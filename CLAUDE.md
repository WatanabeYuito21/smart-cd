# smart-cd — プロジェクト引き継ぎ

## 概要

frecency（頻度×最近さ）ベースのスマートディレクトリジャンプツール。
`zoxide` に似たコンセプトをRustでフルスクラッチ実装する。

**ターゲット環境:** Windows / WSL / Linux（全て同一バイナリ）

---

## 設計方針（決定済み）

| 項目       | 決定内容                                            |
| ---------- | --------------------------------------------------- |
| 言語       | Rust                                                |
| データ保存 | JSON（外部DBなし）                                  |
| 候補選択   | インタラクティブTUI（vim keybind: j/k/g/G/Enter/q） |
| 対応シェル | bash / PowerShell / cmd                             |
| スコア方式 | frecency（訪問回数 × 時間減衰）                     |

---

## CLIインターフェース

```
smart-cd add <path>          # パスをDBに記録（シェルフックから呼ぶ）
smart-cd query <keywords...> # TUIで候補選択 → パスをstdoutに出力
smart-cd list [--paths-only] # スコア順一覧（--paths-onlyは補完用）
smart-cd init bash           # bash統合スクリプトをstdoutに出力
smart-cd init powershell     # PowerShell統合スクリプトをstdoutに出力
smart-cd init cmd            # cmd統合スクリプトをstdoutに出力
```

### シェル統合の仕組み（重要）

外部プロセスは親シェルのカレントディレクトリを変えられない。
そのため **バイナリはパスを返すだけ**。シェル側のラッパー関数が `cd` を実行する。

```bash
# bash での使い方イメージ
eval "$(smart-cd init bash)"
z proj   # → TUIで選択 → シェルがcd実行
```

---

## 予定しているファイル構成

```
smart-cd/
├── CLAUDE.md             ← このファイル
├── SKILL.md              ← Claude Code向けスキル定義
├── Cargo.toml
└── src/
    ├── main.rs           # CLIエントリポイント（clap）
    ├── db.rs             # JSON DB + frecencyスコア計算
    ├── matcher.rs        # キーワードマッチング
    ├── ui.rs             # crossterm製インタラクティブTUI
    └── shell.rs          # シェル統合スクリプト（文字列定数）
```

---

## モジュール設計

### db.rs

```rust
pub struct Entry {
    pub path: String,
    pub visit_count: u32,
    pub last_visited: DateTime<Utc>,
}

impl Entry {
    pub fn score(&self) -> f64 {
        // frecency = visit_count × time_decay
        // time_decay = 1.0 / (1.0 + elapsed_hours × 0.01)
    }
}

pub struct Database {
    pub version: u32,  // 現在は 1
    pub entries: Vec<Entry>,
}
```

**DBファイルパス:**

- Linux/WSL: `~/.local/share/smart-cd/db.json`
- Windows: `%APPDATA%\smart-cd\db.json`

### matcher.rs

- 全キーワードがパスに含まれるAND検索
- 大文字小文字無視
- スコア順を維持して返す

### ui.rs

- **描画先は stderr**（stdout はパス出力専用）
- vim keybind:
  - `j` / `↓` / `Ctrl+n` → 下
  - `k` / `↑` / `Ctrl+p` → 上
  - `g` → 先頭、`G` → 末尾
  - `Enter` → 確定
  - `q` / `Esc` / `Ctrl+c` → キャンセル
- 候補1件のみなら即決定（TUI省略）
- 最大表示件数: 10件（超えた分は「他N件」表示）

### shell.rs

`init_bash()`, `init_powershell()`, `init_cmd()` の3関数。
各シェル統合スクリプトを `&'static str` で返すだけ。

---

## 依存クレート（案）

```toml
clap = { version = "4", features = ["derive"] }
serde = { version = "1", features = ["derive"] }
serde_json = "1"
chrono = { version = "0.4", features = ["serde"] }
crossterm = "0.27"
```

---

## Git ルール

- コミットメッセージに `Co-Authored-By: Claude` 行を**含めない**

---

## 実装ステータス

| ファイル       | 状態                  |
| -------------- | --------------------- |
| Cargo.toml     | 🔲 依存クレート未追加 |
| src/main.rs    | 🔲 スケルトンのみ     |
| src/db.rs      | 🔲 未作成             |
| src/matcher.rs | 🔲 未作成             |
| src/ui.rs      | 🔲 未作成             |
| src/shell.rs   | 🔲 未作成             |

**推奨実装順:** db.rs → matcher.rs → main.rs(add/list) → ui.rs → main.rs(query) → shell.rs

---

## 今後の拡張アイデア（スコープ外）

- `smart-cd remove <path>` — 特定パスを履歴から削除
- `smart-cd clean` — 存在しないパスを一括削除
- fish shell 対応
- fzf 連携オプション
