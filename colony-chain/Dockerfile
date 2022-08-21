# Step 1: Build backend
FROM rust:1.61-alpine AS builder-backend

RUN rustup target add x86_64-unknown-linux-musl
RUN apk add --no-cache musl-dev openssl-dev

WORKDIR /src

COPY /. .

RUN USER=root cargo build -p pdao-polkadot-colony-chain --target x86_64-unknown-linux-musl --release

# Step 2: Compose final image
FROM alpine

WORKDIR /src

COPY --from=builder-backend /src/target/x86_64-unknown-linux-musl/release/pdao-polkadot-colony-chain ./

ENV RUST_LOG=debug 
CMD ["./pdao-polkadot-colony-chain", "8080"]