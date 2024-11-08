FROM rust:1.71 as builder

WORKDIR /app

COPY Cargo.toml Cargo.lock ./
COPY src ./src

RUN cargo test && cargo build --release

FROM rust:1.71

WORKDIR /app

COPY --from=builder /app/target/release/CaramelMQ /app/CaramelMQ

EXPOSE 8080

CMD ["./CaramelMQ"]

