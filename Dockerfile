FROM rust:1-bookworm AS builder

WORKDIR /app
COPY Cargo.toml Cargo.lock ./
COPY src/ ./src/
RUN cargo build --release --locked


FROM gcr.io/distroless/cc-debian12 AS runtime

COPY --from=builder /app/target/release/checkipbot /usr/local/bin/checkipbot

USER 1000:1000
ENTRYPOINT ["/usr/local/bin/checkipbot"]
