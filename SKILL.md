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
