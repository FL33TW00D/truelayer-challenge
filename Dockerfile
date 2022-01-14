FROM rust:1.57.0 AS builder

WORKDIR /app
COPY . .
RUN cargo build --release --bin truelayer 

FROM debian:bullseye-slim AS runtime
WORKDIR /app
RUN apt-get update -y \
    && apt-get install -y --no-install-recommends openssl \
    && apt-get install ca-certificates \
    && apt-get autoremove -y \
    && apt-get clean -y \
    && rm -rf /var/lib/apt/lists/*
COPY --from=builder /app/target/release/truelayer truelayer
COPY configuration configuration
ENTRYPOINT ["./truelayer"]
