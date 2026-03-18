# Etapa de construcción
FROM rust:1.76-alpine AS builder
WORKDIR /app
RUN apk add --no-cache musl-dev
COPY Cargo.toml .
COPY src ./src
RUN cargo build --release

# Etapa final
FROM alpine:latest
WORKDIR /root/
COPY --from=builder /app/target/release/rust-api .
EXPOSE 8080
CMD ["./rust-api"]