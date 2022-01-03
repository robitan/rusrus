FROM debian:latest

RUN apt update -y && apt upgrade -y

RUN apt-get install -y curl

RUN curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y

ENV PATH=/root/.cargo/bin:$PATH

RUN apt-get install -y libssl-dev pkg-config build-essential

RUN cargo install trunk wasm-bindgen-cli

RUN rustup target add wasm32-unknown-unknown
