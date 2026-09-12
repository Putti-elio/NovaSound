# Installation

On Arch Linux or CachyOS, run this command from the server repository root:

```sh
project/scripts/install.sh
```

The script installs missing system packages, installs and selects the stable Rust
toolchain, enables Docker, adds the current user to the `docker` group when
needed, and creates `project/docker/.env` from the committed template when it is
absent. If `project/setup/.env` already exists, the installer leaves it untouched;
manually migrate it to `project/docker/.env` before running server commands. Log
out and back in if it adds you to that group. Then configure the local environment
file before running the server commands in the root README.
