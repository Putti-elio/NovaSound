#!/usr/bin/env bash

set -euo pipefail

if [[ ! -r /etc/os-release ]]; then
  printf '%s\n' 'Unsupported system: /etc/os-release is missing.' >&2
  exit 1
fi

# shellcheck disable=SC1091
source /etc/os-release

if [[ "${ID:-}" != "arch" && "${ID_LIKE:-}" != *"arch"* ]]; then
  printf '%s\n' 'Unsupported distribution. This installer currently supports Arch Linux and CachyOS only.' >&2
  exit 1
fi

if ! command -v sudo >/dev/null 2>&1; then
  printf '%s\n' 'sudo is required to install system packages.' >&2
  exit 1
fi

packages=(
  base-devel
  curl
  docker
  docker-compose
  file
  gtk3
  libayatana-appindicator
  librsvg
  make
  openssl
  webkit2gtk-4.1
)

missing_packages=()
for package in "${packages[@]}"; do
  if ! pacman -Q "$package" >/dev/null 2>&1; then
    missing_packages+=("$package")
  fi
done

if ((${#missing_packages[@]})); then
  sudo pacman -Syu --needed --noconfirm "${missing_packages[@]}"
fi

if ! command -v rustup >/dev/null 2>&1; then
  curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y
fi

export PATH="$HOME/.cargo/bin:$PATH"
rustup toolchain install stable
rustup default stable

if ! cargo tauri --version >/dev/null 2>&1; then
  cargo install tauri-cli --version "^2" --locked
fi

sudo systemctl enable --now docker
if ! groups "$USER" | grep -qw docker; then
  sudo usermod -aG docker "$USER"
  added_to_docker_group=true
else
  added_to_docker_group=false
fi

for command in docker make cargo; do
  command -v "$command" >/dev/null 2>&1
done
docker compose version
cargo tauri --version

if [[ ! -f setup/docker/.env ]]; then
  cp setup/docker/.env_example setup/docker/.env
  printf '%s\n' 'Created setup/docker/.env from .env_example.'
fi

if [[ "$added_to_docker_group" == true ]]; then
  printf '%s\n' 'You were added to the docker group. Log out and back in before using Docker without sudo.'
fi

printf '%s\n' 'Installation complete. Bun, Node, npm, and Vite are intentionally not installed on the host.'
