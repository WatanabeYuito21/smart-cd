---
name: smart-cd
description: >
  smart-cd（Rustで作るfrecencyベースのディレクトリジャンプCLIツール）の
  開発を支援するスキル。
  「smart-cdを実装して」「ビルドエラーを直して」「シェル統合を追加して」
  「テストを書いて」など、このプロジェクトに関する作業が来たら必ずこのスキルを使うこと。
---

# smart-cd 開発スキル

## 作業開始時に必ずやること

1. `CLAUDE.md` を読んで設計方針・実装ステータスを把握する
2. `cargo build` でビルドが通るか確認する
3. ユーザーに「どこから始めるか」を確認する（指示がなければ実装ステータスの上から順）

---

## Git ルール

- コミットメッセージに `Co-Authored-By: Claude` 行を**含めない**
- コミットメッセージは**日本語**で書く
- 先頭に以下のプレフィックスをつける

| プレフィックス | 用途 |
|---|---|
| fix | 既存機能のバグ修正 |
| hotfix | 緊急の変更 |
| add | 新規ファイル・機能の追加 |
| feat | 新機能・新ファイルの追加 |
| update | 既存機能の問題なし修正 |
| change | 仕様変更による既存機能の修正 |
| clean/refactor | コードの整理・改善 |
| improve | コードの改善 |
| disable | 機能の一時無効化 |
| remove/delete | ファイル・機能の削除 |
| rename | ファイル名変更 |
| move | ファイル移動 |
| upgrade | バージョンアップグレード |
| revert | 以前のコミットへの差し戻し |
| docs | ドキュメント修正 |
| style | コーディングスタイル修正 |
| perf | パフォーマンス改善 |
| test | テストコードの追加・修正 |
| chore | ビルドツール・自動生成物、上記に当てはまらない修正 |

---

## 重要ルール

### stderr / stdout の使い分け

`query` コマンドはシェルの `$()` で呼ばれる。混同すると統合が壊れる。

- **stdout** → 選択されたパス（1行のみ）
- **stderr** → TUI描画・エラー・進捗

### クロスプラットフォーム注意点

| 項目     | Windows                      | Linux/WSL                         |
| -------- | ---------------------------- | --------------------------------- |
| DBパス   | `%APPDATA%\smart-cd\db.json` | `~/.local/share/smart-cd/db.json` |
| 環境変数 | `APPDATA`                    | `HOME`                            |

パス操作は `std::path::PathBuf` を使えばほぼ吸収できる。

---

## よく使うコマンド

```bash
cargo build                          # ビルド
cargo check                          # 型チェックのみ（速い）
cargo test                           # テスト実行

# 手動動作確認
./target/debug/smart-cd add /tmp
./target/debug/smart-cd list
./target/debug/smart-cd query tmp    # TUI起動（ターミナルで実行すること）
./target/debug/smart-cd init bash
```

---

## ファイル別の役割

| ファイル         | 役割                              |
| ---------------- | --------------------------------- |
| `src/main.rs`    | CLIコマンドのルーティング（clap） |
| `src/db.rs`      | JSON読み書き・frecencyスコア計算  |
| `src/matcher.rs` | キーワードマッチング              |
| `src/ui.rs`      | TUI描画・キー入力（crossterm）    |
| `src/shell.rs`   | シェル統合スクリプト文字列        |
