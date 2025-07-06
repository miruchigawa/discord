FROM rust:1.87 AS builder 

WORKDIR /app 
COPY . .
RUN cargo build --release

FROM ubuntu:24.04 AS runtime

WORKDIR /app 

COPY --from=builder /app/target/release/discord .
COPY --from=builder /app/assets .

ENTRYPOINT ["./discord"]
