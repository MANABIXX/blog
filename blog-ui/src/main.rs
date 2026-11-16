use gloo_net::http::Request;
use leptos::prelude::*;
use serde::Deserialize;

#[derive(Clone, Debug, Deserialize)]
struct Article {
    title: String,
    description: String,
    url: String,
    date: String,
    #[serde(default)]
    tags: Vec<String>,
    #[serde(default)]
    categories: Vec<String>,
}

#[component]
fn App() -> impl IntoView {
    let (query, set_query) = signal(String::new());
    let (articles, set_articles) = signal(Vec::<Article>::new());
    let (loading, set_loading) = signal(true);
    let (error, set_error) = signal(false);

    Effect::new(move |_| {
        wasm_bindgen_futures::spawn_local(async move {
            match Request::get("/blog/search.json").send().await {
                Ok(response) if response.ok() => {
                    match response.json::<Vec<Article>>().await {
                        Ok(items) => set_articles.set(items),
                        Err(_) => error.set(true),
                    }
                }
                _ => error.set(true),
            }
            set_loading.set(false);
        });
    });

    let filtered = Memo::new(move |_| {
        let q = query.get().trim().to_lowercase();
        let mut rows: Vec<Article> = articles.get().into_iter().filter(|a| {
            q.is_empty() || format!("{} {} {} {}", a.title, a.description, a.tags.join(" "), a.categories.join(" "))
                .to_lowercase().contains(&q)
        }).collect();
        rows.sort_by(|a,b| b.date.cmp(&a.date));
        rows
    });

    view! {
        <header class="top">
            <a class="brand" href="/blog/">"MANABIXX"<span>" BLOG"</span></a>
            <nav><a href="/blog/">"トップ"</a><a href="/blog/archives/">"記事一覧"</a></nav>
        </header>
        <main class="app-shell">
            <p class="eyebrow">"DISCOVER KNOWLEDGE"</p>
            <h1>"記事を探す"</h1>
            <p class="lead">"勉強法・学習の工夫を、キーワードから探せます。"</p>
            <input class="app-search" type="search" placeholder="記事名・教科・キーワードで検索"
                prop:value=move || query.get()
                on:input=move |ev| set_query.set(event_target_value(&ev)) />
            <p class="app-status">
                {move || if loading.get() { "記事データを読み込んでいます…" } else if error.get() { "記事データを読み込めませんでした。時間をおいて再読み込みしてください。" } else { "" }}
                {move || if !loading.get() && !error.get() { format!("{} 件の記事", filtered.get().len()) } else { String::new() }}
            </p>
            <section class="app-results">
                <For each=move || filtered.get() key=|a| a.url.clone() let:item>
                    <a class="app-result" href=item.url.clone()>
                        <span class="app-meta">{item.date.clone()}</span>
                        <h2>{item.title.clone()}</h2>
                        <p>{item.description.clone()}</p>
                    </a>
                </For>
            </section>
            <a class="app-back" href="/blog/">"← ブログトップへ戻る"</a>
        </main>
    }
}

fn main() {
    console_error_panic_hook::set_once();
    mount_to_body(App);
}
