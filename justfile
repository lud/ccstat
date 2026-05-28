build:
    cargo build --release

install: build
    cp target/release/ccstat ~/.local/bin/ccstat

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

