build:
    cargo build --release

install: build
    cp target/release/ccstat ~/.local/bin/ccstat
