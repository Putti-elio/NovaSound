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
  make
  openssl
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

if [[ ! -f project/docker/.env ]]; then
  if [[ -f project/setup/.env ]]; then
    printf '%s\n' 'Existing project/setup/.env was left untouched. Manually migrate it to project/docker/.env before running server commands.' >&2
  else
    cp project/docker/.env.example project/docker/.env
    printf '%s\n' 'Created project/docker/.env from .env.example.'
  fi
fi

if [[ "$added_to_docker_group" == true ]]; then
  printf '%s\n' 'You were added to the docker group. Log out and back in before using Docker without sudo.'
fi

printf '%s\n' 'Backend installation complete.'
