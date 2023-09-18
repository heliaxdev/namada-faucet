build:
    cacrgo build

build-release:
    cargo build --release

misc:
    cargo clippy --fix --allow-dirty
    cargo fmt