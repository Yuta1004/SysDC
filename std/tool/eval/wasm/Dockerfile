FROM rust:1.63.0

WORKDIR /workdir

COPY ./Cargo.toml ./Cargo.toml
COPY ./Cargo.core.toml ./core/Cargo.toml

RUN curl https://rustwasm.github.io/wasm-pack/installer/init.sh -sSf | sh && \
    mkdir src && \
    touch src/lib.rs && \
    mkdir -p core/src && \
    touch core/src/lib.rs && \
    rustup target add wasm32-unknown-unknown && \
    cargo build
