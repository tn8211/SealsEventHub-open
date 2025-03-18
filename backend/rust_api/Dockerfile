FROM rust:latest AS builder

WORKDIR /app

COPY . .

RUN cargo build --release

FROM rust:latest

ENV ROCKET_CONFIG=/usr/local/bin/configs/Rocket.toml

WORKDIR /usr/local/bin

COPY --from=builder /app/target/release/rust_api .
COPY --from=builder /app/cert.pem /app/private/cert.pem
COPY --from=builder /app/ukey.pem /app/private/ukey.pem
COPY --from=builder /app/Rocket.toml /usr/local/bin/configs/Rocket.toml

EXPOSE 8000

CMD ["./rust_api"]
