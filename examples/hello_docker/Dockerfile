FROM rust:latest

RUN apt-get update && apt-get install -y netcat

WORKDIR /usr/src/app

COPY . .

RUN cargo install --git  https://github.com/aktitarov/wait_for wait_for

RUN cargo build --release
RUN cargo install --path .