ARG RUST_VERSION=1.98.0

FROM rust:${RUST_VERSION}-trixie AS builder

WORKDIR /app

COPY Cargo.toml Cargo.lock ./
COPY .cargo ./.cargo
COPY src ./src

RUN cargo build --release --locked

FROM gcr.io/distroless/cc-debian13:nonroot

WORKDIR /app

COPY --from=builder /app/target/x86_64-unknown-linux-gnu/release/hiraishin hiraishin

ENTRYPOINT [ "./hiraishin" ]
