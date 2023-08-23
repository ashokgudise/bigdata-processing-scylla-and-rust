FROM rust:latest

WORKDIR /usr/src/producer

COPY . .

RUN cargo build --release

CMD ["target/release/producer"]  

# docker build -t ashok-demos/scylla-db-rust-producer . 