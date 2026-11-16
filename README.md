# MANABIXX Blog

MANABIXX本体とは独立して運用する、学習法・学習習慣・受験準備のためのブログです。

- 公開URL: https://manabixx.github.io/blog/
- 記事UI: Rust + Leptos（WebAssembly）
- コンテンツ生成・検索DB: Rust
- データベース: SQLite + FTS5
- 配信: GitHub Pages（静的サイト）
- 記事本文: Markdown + YAML front matter

## 構成

- `content/posts/`: 公開記事
- `content/study/`: 個人向けの非公開扱いページ（noindex、RSS・サイトマップから除外）
- `src/main.rs`: Markdown記事をHTMLに変換し、SQLite検索DB・JSON検索データ・RSS・サイトマップを生成
- `blog-ui/`: Leptos製の検索アプリ（`/blog/app/`）
- `static/assets/site.css`: ブログ共通デザイン
- `public/data/articles.sqlite`: ビルド時に生成するSQLite全文検索DB

GitHub Pagesは静的配信のため、SQLiteはビルド時の正本・検索インデックスとして使います。LeptosのブラウザーUIには同じDBから書き出したJSONを渡します。サーバーを必要とする書き込みAPIやログイン機能は持たせません。

## ローカルビルド

```bash
cargo run --release -- content public https://manabixx.github.io/blog/
rustup target add wasm32-unknown-unknown
cargo install trunk --locked
cd blog-ui
trunk serve
```

## 編集・公開

記事は `content/posts/` にMarkdownで追加します。公開記事には `title`、`date`、`description` を設定してください。個人向けページは `content/study/` に置き、公開記事と混ぜないでください。

`main` へのpush、またはActionsの手動実行でRustの静的生成とLeptosのWASMビルドを行い、GitHub Pagesへデプロイします。Actionsが成功するまでは公開完了とは扱いません。

## 編集品質

1. 読者と悩みを冒頭で具体的に示す。
2. 結論を先に示し、具体例・再現できる手順を含める。
3. 失敗例と修正方法を示す。
4. 制度・数値・製品など変動情報は公式一次資料を優先し、確認日とリンクを記載する。
5. 中学生にも伝わる自然な日本語を使い、専門用語には説明を添える。
6. 既存記事の言い換えを避け、薄い記事を量産しない。
7. 個人名・個人を特定できる情報・非公開の学習記録を掲載しない。
8. 公開前にリンク、画像alt、見出し、説明の矛盾を確認する。
