set windows-shell := ["powershell.exe", "-NoLogo", "-NoProfile", "-Command"]

build:
    cargo build --release

[unix]
install: build
    cp target/release/ccstat ~/.local/bin/ccstat

[windows]
install: build
    New-Item -ItemType Directory -Force "$env:USERPROFILE/.local/bin" | Out-Null
    Copy-Item -Force target/release/ccstat.exe "$env:USERPROFILE/.local/bin/ccstat.exe"

# kind: major | minor | patch | <explicit version>
release kind:
    cargo release {{kind}} --execute

format:
  cargo fmt --all

_cargo_clippy:
  cargo clippy --all-targets -- -D warnings

_check_build:
  cargo build --release

test:
  cargo test

_git_status:
  git status

check: format test _check_build _cargo_clippy _git_status

