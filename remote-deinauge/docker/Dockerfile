
FROM rust:latest

WORKDIR /usr/src/rust-app

COPY . .

RUN cargo build --release

EXPOSE 8888

CMD ["./target/release/remote-deinauge"]
