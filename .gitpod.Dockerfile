FROM gitpod/workspace-full:latest

USER root

RUN sudo rustup target add wasm32-unknown-unknown && sudo cargo install trunk
RUN echo "Successfully installed the Packages"

