FROM rust:latest AS builder
ARG SQLX_OFFLINE=true

RUN rustup toolchain install nightly
RUN rustup default nightly

COPY . .
RUN cargo build --release

CMD ["./target/release/atomicflow-operator"]