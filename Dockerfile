FROM rust:slim-bullseye

WORKDIR /usr/src/app

COPY . .

RUN cargo build --release

CMD ["cargo", "run"]
