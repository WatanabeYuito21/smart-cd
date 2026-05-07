# smart-cd

frecency（頻度 × 最近さ）ベースのスマートディレクトリジャンプツール。
よく使うディレクトリをキーワードで素早く移動できます。

## Features

- **frecency スコア** — 訪問回数と最終訪問時刻を組み合わせたランキング
- **インタラクティブ TUI** — vim キーバインドで候補を選択
- **クロスプラットフォーム** — bash / PowerShell / cmd / fish に対応
- **シンプル** — 外部 DB 不要。JSON 1 ファイルで完結

## Requirements

- Rust / Cargo **1.85 以上**（edition 2024 を使用しているため）

バージョンが古い場合は以下で更新できます:

```bash
rustup update
```

## Installation

```bash
cargo install --path .
```

## Shell Integration

### bash

`~/.bashrc` に以下を追加:

```bash
eval "$(smart-cd init bash)"
```

### PowerShell

`$PROFILE` に以下を追加:

```powershell
Invoke-Expression (smart-cd init powershell)
```

### fish

`~/.config/fish/config.fish` に以下を追加:

```fish
smart-cd init fish | source
```

### cmd

コマンドプロンプトの起動時に実行されるバッチファイルに追加:

```bat
for /f "delims=" %i in ('smart-cd init cmd') do @%i
```

## Usage

### ディレクトリジャンプ

```bash
z proj        # "proj" を含むディレクトリへジャンプ
z doc rust    # "doc" と "rust" の両方を含むディレクトリへジャンプ
```

候補が1件なら即移動。複数件ならインタラクティブ TUI で選択します。

### TUI キーバインド

| キー | 動作 |
|---|---|
| `j` / `↓` / `Ctrl+n` | 下へ |
| `k` / `↑` / `Ctrl+p` | 上へ |
| `g` | 先頭へ |
| `G` | 末尾へ |
| `Enter` | 確定 |
| `q` / `Esc` / `Ctrl+c` | キャンセル |

### サブコマンド

```
smart-cd add <path>           パスを履歴に記録（シェルフックが自動で呼ぶ）
smart-cd query <keywords...>  TUI で候補選択 → パスを stdout に出力
smart-cd list                 スコア順で一覧表示
smart-cd list --paths-only    パスのみ出力（補完用）
smart-cd remove <path>        特定パスを履歴から削除
smart-cd clean                存在しないパスを一括削除
smart-cd init bash            bash 統合スクリプトを出力
smart-cd init powershell      PowerShell 統合スクリプトを出力
smart-cd init cmd             cmd 統合スクリプトを出力
smart-cd init fish            fish 統合スクリプトを出力
```

## How It Works

ディレクトリを移動するたびにシェルフックが `smart-cd add` を呼び出し、履歴を更新します。

スコアは以下の式で計算されます:

```
score = visit_count × (1 / (1 + 経過時間(h) × 0.01))
```

訪問回数が多く、かつ最近使ったディレクトリほど上位に表示されます。

## DB ファイルパス

| 環境 | パス |
|---|---|
| Linux / WSL | `~/.local/share/smart-cd/db.json` |
| Windows | `%APPDATA%\smart-cd\db.json` |
