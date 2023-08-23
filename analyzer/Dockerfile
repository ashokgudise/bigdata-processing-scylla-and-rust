FROM rust:latest

WORKDIR /usr/src/analyzer

COPY . .

RUN cargo build --release

CMD ["target/release/analyzer"]  

# docker build -t ashok-demos/scylla-db-rust-analyzer . 