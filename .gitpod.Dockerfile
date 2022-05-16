FROM gitpod/workspace-full:2022-05-08-14-31-53

USER root

RUN sudo rustup target add wasm32-unknown-unknown && sudo cargo install trunk
RUN echo "Successfully installed the Packages"

