# MANABIXX Blog

勉強法・学習習慣・受験準備を扱う独立ブログです。

- URL: https://manabixx.github.io/blog/
- Framework: Hugo Extended
- Theme: Hugo Theme Stack v4.0.3（Hugo Modulesで取得）
- Hosting: GitHub Pages

## Preview

```bash
hugo server -D
```

## Editorial policy

読者が記事を読んだあとに「何を理解できたか」「次に何をすればよいか」が残る記事を公開します。記事数を増やすことより、内容の正確さ・具体性・読みやすさを優先します。

### 必須品質基準

1. **読者と悩みを明確にする。** 誰のどんな困りごとを解決する記事なのか、冒頭で具体的に示す。
2. **結論を先に示す。** 抽象的な前置きを長く続けず、記事で伝える要点を最初に説明する。
3. **具体例を必ず入れる。** 勉強法なら実際の問題・答案・ノート例・時間配分など、読者が再現できる例を示す。一般論だけで記事を終えない。
4. **手順を省略しない。** 「復習しよう」「計画を立てよう」だけで済ませず、何を、どの順番で、どの程度行うかを書く。
5. **失敗例と修正方法を示す。** ありがちな間違い、なぜうまくいかないか、どう直すかを具体的に説明する。
6. **根拠と出典を確認する。** 学習法の効果を断定しない。入試・制度・製品など変動する情報は公式一次資料を優先し、確認日とリンクを記載する。確認できない情報は推測で埋めない。
7. **中学生にも伝わる言葉を使う。** 専門用語には短い説明を添え、不要なカタカナ語や難解な言い回しを避ける。
8. **内容の重複を避ける。** 既存記事と同じ結論・構成を使い回さず、新しい記事ならではの情報や視点を用意する。
9. **自然な日本語にする。** 定型的な導入・結び、同じ言い回しの連発、過剰な箇条書き、意味の薄い「まとめ」を避ける。AIが書いたような不自然な文章をそのまま公開しない。
10. **公開前に読み直す。** 誤字、リンク切れ、画像の内容、見出しと本文の一致、説明の矛盾を確認する。

### 公開前チェック

- [ ] 冒頭で読者の悩みと記事の結論が分かる
- [ ] 読者が実行できる具体例・手順がある
- [ ] 抽象論だけで終わっていない
- [ ] 事実・数字・制度に根拠と出典がある
- [ ] 他の記事の言い換えになっていない
- [ ] 中学生が読んでも意味を追える
- [ ] 最後に、読者が次に取る行動が明確になっている

基準を満たさない記事は、公開を急がず書き直します。薄い記事を大量に追加することは禁止です。

## Privacy

- 個人名、個人を特定できる情報、非公開の学習記録は掲載しません。
- 個人向けの学習メモは一般公開記事と分離し、検索・RSS・サイトマップに混入させません。

## Publishing

GitHub Actions builds the site with Hugo Extended and Hugo Theme Stack v4, then deploys it to GitHub Pages.

For the first publication, open repository **Settings → Pages → Build and deployment** and set **Source** to **GitHub Actions**. After that, pushes to `main` publish automatically.

A change is considered published only after the `Deploy MANABIXX Blog` workflow completes successfully.
