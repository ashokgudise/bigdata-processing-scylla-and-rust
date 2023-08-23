FROM rust:latest

WORKDIR /usr/src/processor

COPY . .

RUN cargo build --release

CMD ["target/release/processor"]  

# docker build -t ashok-demos/scylla-db-rust-processor . 