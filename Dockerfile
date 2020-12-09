FROM rust:1.48

WORKDIR /usr/src/app
COPY . .

RUN cargo install --path .

CMD ["app"]