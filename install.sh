#!/bin/sh
set -e

REPO="WatanabeYuito21/smart-cd"
INSTALL_DIR="${HOME}/.local/bin"

os=$(uname -s)
arch=$(uname -m)

case "$os" in
  Linux)
    case "$arch" in
      x86_64) target="linux-x86_64" ;;
      *) echo "Error: unsupported architecture: $arch" >&2; exit 1 ;;
    esac
    ;;
  Darwin)
    case "$arch" in
      x86_64)  target="macos-x86_64" ;;
      arm64)   target="macos-aarch64" ;;
      *) echo "Error: unsupported architecture: $arch" >&2; exit 1 ;;
    esac
    ;;
  *)
    echo "Error: unsupported OS: $os (use cargo install for Windows)" >&2
    exit 1
    ;;
esac

latest=$(curl -sL "https://api.github.com/repos/${REPO}/releases/latest" \
  | grep '"tag_name"' | head -1 | cut -d'"' -f4)

if [ -z "$latest" ]; then
  echo "Error: failed to fetch latest release" >&2
  exit 1
fi

url="https://github.com/${REPO}/releases/download/${latest}/smart-cd-${target}.tar.gz"

echo "Downloading smart-cd ${latest} for ${target}..."
curl -sL "$url" | tar xz -C /tmp

mkdir -p "$INSTALL_DIR"
mv /tmp/smart-cd "$INSTALL_DIR/smart-cd"
chmod +x "$INSTALL_DIR/smart-cd"

echo "Installed to ${INSTALL_DIR}/smart-cd"

# PATH が通っていなければシェルプロファイルに追記する
case ":$PATH:" in
  *":${INSTALL_DIR}:"*)
    ;;
  *)
    PATH_LINE="export PATH=\"\$HOME/.local/bin:\$PATH\""

    shell_name=$(basename "${SHELL:-sh}")
    case "$shell_name" in
      zsh)   profile="${HOME}/.zshrc" ;;
      bash)  profile="${HOME}/.bashrc" ;;
      fish)
        mkdir -p "${HOME}/.config/fish/conf.d"
        profile="${HOME}/.config/fish/conf.d/smart-cd-path.fish"
        PATH_LINE="fish_add_path \$HOME/.local/bin"
        ;;
      *)     profile="${HOME}/.profile" ;;
    esac

    # 既に書いてあれば追記しない
    if ! grep -qF ".local/bin" "$profile" 2>/dev/null; then
      printf '\n%s\n' "$PATH_LINE" >> "$profile"
      echo "Added PATH to ${profile}"
    fi

    echo "Restart your shell or run: source ${profile}"
    ;;
esac
