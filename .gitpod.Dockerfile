FROM gitpod/workspace-full:2022-05-08-14-31-53

RUN rustup target add wasm32-unknown-unknown \
cargo install trunk