# MANABIXX Blog

MANABIXX本体とは独立した記事発信専用ブログ。旧Rust/Leptos構成・既存記事を引き継がず、Hexoでゼロから再構築。

- 公開: https://manabixx.github.io/blog/
- CMS/生成: Hexo 7
- 編集: hexo-admin
- 記事: Markdown + YAML Front Matter
- テーマ: MANABIXX専用オリジナル
- 配信: GitHub Pages + Actions

## 管理画面
Node.js 20以上を用意してリポジトリ直下で実行:

```bash
npm install
npm run admin
```

http://localhost:4000/blog/admin/ を開く。通常プレビューは http://localhost:4000/blog/ 。記事を編集・保存したらGitでコミット・pushするとActionsが生成・公開する。

**重要:** GitHub Pagesは静的ホスティングのため、公開URL上でhexo-adminの編集APIは動かない。管理画面はローカルで利用し、管理サーバーを無認証でインターネットへ公開しない。

## 品質方針
読者の悩みを一つに絞り、結論・具体例・再現手順・失敗例・改善策を書く。中学生にも伝わる自然な日本語。制度・数値・製品仕様は一次情報で確認。薄い記事の量産、個人情報や非公開学習記録の掲載は禁止。公開前にリンク・見出し・alt・スマホ表示を確認。

- `source/_posts/`: 公開記事
- `source/about/`: 固定ページ
- `themes/manabixx/`: 専用テーマ
- `.github/workflows/deploy.yml`: CI/CD

`npm run build` / `npm run server` / `npm run admin` / `npx hexo new "記事タイトル"`
