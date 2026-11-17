# MANABIXX Blog

MANABIXXの学び・技術メディア。ブログ本体と要点まとめを別エンジンで運用します。

## 公開URL

- ブログ（Zola）: https://manabixx.github.io/blog/
- 要点まとめ（Quartz）: https://manabixx.github.io/blog/summary/

## 技術構成

### Blog
- Zola（Rust製静的サイトジェネレーター）
- Tera templates
- Markdown
- CSS
- RSS / sitemap / 検索インデックス
- GitHub Pages

### Summary
- Quartz v4
- TypeScript / Markdown
- Wikiリンク・バックリンク・グラフ表示・全文検索
- GitHub Pages（ブログと同じデプロイ成果物内の /summary/）

## 更新方法

- ブログ記事: `content/` にMarkdownを追加
- 要点まとめ: `summary/content/` にMarkdownを追加
- 要点まとめ内の関連ページは `[[ページ名]]` で接続
- mainへpushするとGitHub Actionsが両方をビルドして公開

## 編集方針

記事は読者の疑問を具体的に解決する。結論、具体例、再現手順、つまずきやすい点を含め、根拠のない断定や薄い記事の量産を避ける。要点まとめは短く正確にし、定義・公式・条件・例外を整理して関連知識へリンクする。

既存のHexo記事は新サイトへ移行せず、旧ファイルとして保管しています。
