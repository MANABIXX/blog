use chrono::{DateTime, FixedOffset, NaiveDate, Utc};
use pulldown_cmark::{html, Options, Parser};
use serde::Serialize;
use serde_yaml::Value;
use std::{env, fs, path::{Path, PathBuf}};
use walkdir::WalkDir;

#[derive(Clone, Serialize)]
struct Article {
    title: String, slug: String, description: String, date: String,
    iso_date: String, section: String, url: String, html: String,
    image: String, image_alt: String, categories: Vec<String>, tags: Vec<String>,
    noindex: bool,
}

fn value<'a>(v: &'a Value, key: &str) -> Option<&'a Value> {
    v.as_mapping()?.get(Value::String(key.to_owned()))
}
fn string(v: Option<&Value>) -> String {
    match v {
        Some(Value::String(s)) => s.clone(),
        Some(Value::Number(n)) => n.to_string(),
        Some(Value::Tagged(t)) => string(Some(&t.value)),
        Some(x) => x.as_str().unwrap_or("").to_owned(),
        None => String::new(),
    }
}
fn list(v: Option<&Value>) -> Vec<String> {
    match v {
        Some(Value::Sequence(xs)) => xs.iter().map(|x| string(Some(x))).filter(|s| !s.is_empty()).collect(),
        Some(Value::String(s)) => vec![s.clone()],
        _ => vec![],
    }
}
fn esc(s: &str) -> String {
    s.replace('&', "&amp;").replace('<', "&lt;").replace('>', "&gt;")
     .replace('"', "&quot;").replace('\'', "&#39;")
}
fn slugify(path: &Path, meta: &Value) -> String {
    let explicit = string(value(meta, "slug"));
    if !explicit.is_empty() { return explicit; }
    let stem = path.file_stem().unwrap_or_default().to_string_lossy();
    let parts: Vec<_> = stem.splitn(4, '-').collect();
    let name = if parts.len() == 4
        && parts[0].len() == 4 && parts[0].chars().all(|c| c.is_ascii_digit())
        && parts[1].len() == 2 && parts[1].chars().all(|c| c.is_ascii_digit())
        && parts[2].len() == 2 && parts[2].chars().all(|c| c.is_ascii_digit()) {
        parts[3]
    } else { &stem };
    name.to_lowercase().replace(' ', "-")
}
fn date_fields(raw: &str) -> (String, String) {
    if let Ok(d) = DateTime::parse_from_rfc3339(raw) {
        return (d.format("%Y年%m月%d日").to_string(), d.to_rfc3339());
    }
    if let Ok(d) = NaiveDate::parse_from_str(raw, "%Y-%m-%d") {
        return (d.format("%Y年%m月%d日").to_string(), format!("{}T00:00:00Z", d));
    }
    let today = Utc::now().format("%Y-%m-%d").to_string();
    (today.clone(), format!("{today}T00:00:00Z"))
}
fn markdown(source: &str) -> String {
    let mut opts = Options::empty();
    opts.insert(Options::ENABLE_TABLES | Options::ENABLE_STRIKETHROUGH | Options::ENABLE_FOOTNOTES);
    let parser = Parser::new_ext(source, opts);
    let mut out = String::new();
    html::push_html(&mut out, parser);
    out
}
fn read_article(path: &Path, root: &Path, base: &str) -> Option<Article> {
    let raw = fs::read_to_string(path).ok()?;
    let trimmed = raw.trim_start();
    let rest = trimmed.strip_prefix("---\n")?;
    let (front, body) = rest.split_once("\n---")?;
    let meta: Value = serde_yaml::from_str(front).ok()?;
    let title = string(value(&meta, "title"));
    if title.is_empty() { return None; }
    let rel = path.strip_prefix(root).ok()?;
    let parts: Vec<_> = rel.components().map(|c| c.as_os_str().to_string_lossy().to_string()).collect();
    let is_study = parts.first().map(|s| s == "study").unwrap_or(false);
    let section = if is_study { parts.get(1).cloned().unwrap_or_else(|| "misc".into()) } else { "posts".into() };
    let slug = slugify(path, &meta);
    let (date, iso_date) = date_fields(&string(value(&meta, "date")));
    let url = if is_study { format!("/study/{section}/{slug}/") } else { format!("/p/{slug}/") };
    let image = string(value(&meta, "image"));
    let image = if image.starts_with("http") { image } else if image.is_empty() { String::new() } else { format!("{}{}", base.trim_end_matches('/'), if image.starts_with('/') { image } else { format!("/{image}") }) };
    let noindex = is_study || string(value(&meta, "noindex")) == "true";
    Some(Article {
        title, slug, description: string(value(&meta, "description")), date, iso_date,
        section, url, html: markdown(body.trim()), image,
        image_alt: string(value(&meta, "imageAlt")),
        categories: list(value(&meta, "categories")), tags: list(value(&meta, "tags")), noindex,
    })
}
fn shell(title: &str, desc: &str, body: &str, base: &str, noindex: bool) -> String {
    let robots = if noindex { r#"<meta name="robots" content="noindex,nofollow">"# } else { "" };
    format!(r#"<!doctype html><html lang="ja"><head><meta charset="utf-8"><meta name="viewport" content="width=device-width,initial-scale=1"><meta name="description" content="{}">{}<title>{} | MANABIXX Blog</title><link rel="canonical" href="{}"><link rel="stylesheet" href="{}assets/site.css"><link rel="alternate" type="application/rss+xml" title="MANABIXX Blog" href="{}index.xml"></head><body><header class="top"><a class="brand" href="{}">MANABIXX<span> BLOG</span></a><nav><a href="{}">記事</a><a href="{}about/">このブログについて</a><a href="{}archives/">アーカイブ</a></nav></header><main>{}</main><footer>MANABIXX Blog <span>学びを、わかるだけで終わらせない。</span></footer></body></html>"#,
        esc(desc), robots, esc(title), format!("{}{}", base.trim_end_matches('/'), "/"), base, base, base, base, base, body)
}
fn card(a: &Article, base: &str) -> String {
    let image = if a.image.is_empty() { String::new() } else { format!(r#"<img class="thumb" src="{}" alt="{}" loading="lazy">"#, esc(&a.image), esc(&a.image_alt)) };
    format!(r#"<article class="card">{}<div><p class="meta">{}</p><h2><a href="{}{}">{}</a></h2><p>{}</p><a class="more" href="{}{}">続きを読む →</a></div></article>"#,
        image, a.date, base.trim_end_matches('/'), a.url, esc(&a.title), esc(&a.description), base.trim_end_matches('/'), a.url)
}
fn write(path: &Path, data: &str) -> Result<(), Box<dyn std::error::Error>> {
    if let Some(parent) = path.parent() { fs::create_dir_all(parent)?; }
    fs::write(path, data)?; Ok(())
}
fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args: Vec<String> = env::args().collect();
    let source = PathBuf::from(args.get(1).map(String::as_str).unwrap_or("content"));
    let output = PathBuf::from(args.get(2).map(String::as_str).unwrap_or("public"));
    let base = args.get(3).map(String::as_str).unwrap_or("https://manabixx.github.io/blog/");
    if output.exists() { fs::remove_dir_all(&output)?; }
    fs::create_dir_all(&output)?;
    let mut articles = Vec::new();
    for entry in WalkDir::new(&source).into_iter().filter_map(Result::ok) {
        if entry.file_type().is_file() && entry.path().extension().is_some_and(|e| e == "md") {
            if let Some(a) = read_article(entry.path(), &source, base) { articles.push(a); }
        }
    }
    articles.sort_by(|a,b| b.iso_date.cmp(&a.iso_date));
    let public: Vec<_> = articles.iter().filter(|a| !a.noindex && a.section == "posts").collect();
    let study: Vec<_> = articles.iter().filter(|a| a.noindex).collect();

    let cards = public.iter().map(|a| card(a, base)).collect::<String>();
    let home = format!(r#"<section class="hero"><p class="eyebrow">LEARN WITH INTENTION</p><h1>勉強を、<br>ちゃんと自分の力に。</h1><p>覚える、考える、解き直す。学びを前に進めるための実践的な記事を届けます。</p></section><section class="section-head"><div><p class="eyebrow">LATEST ARTICLES</p><h2>新着記事</h2></div></section><div class="cards">{}</div>"#, cards);
    write(&output.join("index.html"), &shell("学びを、自分の力に", "勉強法・学習の工夫を具体例とともに紹介します。", &home, base, false))?;

    for a in &articles {
        let mut body = format!(r#"<article class="article"><p class="meta">{}</p><h1>{}</h1><p class="lead">{}</p>{}<div class="prose">{}</div></article>"#,
            a.date, esc(&a.title), esc(&a.description),
            if a.image.is_empty() { String::new() } else { format!(r#"<img class="cover" src="{}" alt="{}">"#, esc(&a.image), esc(&a.image_alt)) },
            a.html);
        if a.noindex { body = format!(r#"<div class="private-note">個人向けコンテンツ</div>{}"#, body); }
        let page = shell(&a.title, &a.description, &body, base, a.noindex);
        let dest = output.join(a.url.trim_start_matches('/')).join("index.html");
        write(&dest, &page)?;
    }

    let archive_cards = public.iter().map(|a| format!(r#"<li><time>{}</time><a href="{}{}">{}</a></li>"#, a.date, base.trim_end_matches('/'), a.url, esc(&a.title))).collect::<String>();
    let archive = format!(r#"<section class="page-head"><p class="eyebrow">ARCHIVE</p><h1>記事一覧</h1></section><ul class="archive">{}</ul>"#, archive_cards);
    write(&output.join("archives/index.html"), &shell("記事一覧", "MANABIXX Blogの記事一覧", &archive, base, false))?;
    let about = r#"<section class="page-head"><p class="eyebrow">ABOUT</p><h1>このブログについて</h1></section><div class="prose"><p>MANABIXX Blogは、勉強の工夫や学び方を、具体例と実践できる手順で届ける学習ブログです。</p><p>抽象的な精神論ではなく、今日の学習で試せる方法を大切にします。</p></div>"#;
    write(&output.join("about/index.html"), &shell("このブログについて", "MANABIXX Blogについて", about, base, false))?;

    let search_data: Vec<_> = public.iter().map(|a| serde_json::json!({"title":a.title,"description":a.description,"url":format!("{}{}",base.trim_end_matches('/'),a.url),"date":a.date,"tags":a.tags,"categories":a.categories})).collect();
    write(&output.join("search.json"), &serde_json::to_string(&search_data)?)?;

    let mut rss = String::from(r#"<?xml version="1.0" encoding="UTF-8"?><rss version="2.0"><channel><title>MANABIXX Blog</title><link>"#);
    rss.push_str(base); rss.push_str("</link><description>勉強の工夫と学び方</description>");
    for a in public.iter().take(30) {
        rss.push_str(&format!("<item><title>{}</title><link>{}{}</link><description>{}</description><pubDate>{}</pubDate></item>",
            esc(&a.title), base.trim_end_matches('/'), a.url, esc(&a.description), a.iso_date));
    }
    rss.push_str("</channel></rss>");
    write(&output.join("index.xml"), &rss)?;

    let mut sitemap = format!(r#"<?xml version="1.0" encoding="UTF-8"?><urlset xmlns="http://www.sitemaps.org/schemas/sitemap/0.9"><url><loc>{}</loc></url>"#, base);
    for a in &public { sitemap.push_str(&format!("<url><loc>{}{}</loc><lastmod>{}</lastmod></url>", base.trim_end_matches('/'), a.url, &a.iso_date[..10.min(a.iso_date.len())])); }
    sitemap.push_str("</urlset>");
    write(&output.join("sitemap.xml"), &sitemap)?;
    write(&output.join("robots.txt"), &format!("User-agent: *\nAllow: /\nSitemap: {}sitemap.xml\n", base))?;

    let static_dir = Path::new("static");
    if static_dir.exists() {
        for entry in WalkDir::new(static_dir).into_iter().filter_map(Result::ok) {
            if entry.file_type().is_file() {
                let rel = entry.path().strip_prefix(static_dir)?;
                let dest = output.join(rel);
                if let Some(parent) = dest.parent() { fs::create_dir_all(parent)?; }
                fs::copy(entry.path(), dest)?;
            }
        }
    }
    let _ = study;
    println!("Rust build complete: {} public articles, {} unlisted study pages", public.len(), articles.len() - public.len());
    Ok(())
}
