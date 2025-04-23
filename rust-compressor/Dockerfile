FROM rust:1.67-bookworm

WORKDIR /app

COPY ./target/debug/rust-compressor .

ENTRYPOINT ["/app/rust-compressor"]
