FROM gitpod/workspace-full:2022-05-08-14-31-53

RUN sudo rustup target add wasm32-unknown-unknown && sudo cargo install trunk 

