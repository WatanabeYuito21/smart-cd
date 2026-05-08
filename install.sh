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

case ":$PATH:" in
  *":${INSTALL_DIR}:"*) ;;
  *) echo "Note: add the following to your shell profile: export PATH=\"\$HOME/.local/bin:\$PATH\"" ;;
esac
