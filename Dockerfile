FROM rust:1.58 as builder

WORKDIR /app

COPY . .
RUN cargo build --release

FROM debian:buster-slim
COPY --from=builder /app/target/release/thumbtaro .
CMD ["./thumbtaro"]
