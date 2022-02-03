FROM ekidd/rust-musl-builder:stable as builder

WORKDIR /app

COPY . .
RUN cargo build --release

FROM debian:buster-slim
COPY --from=builder /home/rust/src/target/x86_64-unknown-linux-musl/release/thumbtaro .
CMD ["./thumbtaro"]
