build:
    cargo build --release

install: build
    cp target/release/ccstat ~/.local/bin/ccstat

# kind: major | minor | patch | <explicit version>
release kind:
    cargo release {{kind}} --execute
