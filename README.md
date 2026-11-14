# MANABIXX Blog

勉強法・学習習慣・受験準備・モチベーションを扱う独立ブログです。

- URL: https://manabixx.github.io/blog/
- Framework: Hugo Extended
- Theme: Hugo Theme Stack v4.0.3（Hugo Modulesで取得）
- Hosting: GitHub Pages

## Preview

```bash
hugo server -D
```

## Editorial policy

- 読者が実践できる具体的な学習の工夫を紹介します。
- 個人名、個人を特定できる情報、非公開の学習記録は掲載しません。
- 入試情報など変動する情報は公式一次資料を確認し、確認日と出典を記載します。
- 外部の画像・資料を使用する場合は、利用条件と出典を確認します。

## Publishing

GitHub Actions builds the site with Hugo Extended and Hugo Theme Stack v4, then deploys it to GitHub Pages.

For the first publication, open repository **Settings → Pages → Build and deployment** and set **Source** to **GitHub Actions**. After that, pushes to `main` publish automatically.

The site is considered published only after the `Deploy MANABIXX Blog` workflow completes successfully.
