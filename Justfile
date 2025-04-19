setup:
    sudo apt-get install g++ pkg-config libx11-dev libasound2-dev libudev-dev
    cargo install cargo-llvm-cov
    rustup component add llvm-tools-preview

lint:
    cargo clippy --all

build:
    cargo build

test:
    RUST_BACKTRACE=full cargo llvm-cov --workspace --html --open

publish:
    #!/bin/bash
    version="$(cat Cargo.toml | grep '^version = ' | cut -d ' ' -f3 | sed 's/\"//g')"
    git tag -a $version -m "Version ${version}"
    git push origin ${version}
    cargo publish
