FROM rust:latest

WORKDIR /usr/src/monadium
COPY . .

RUN cargo install --path .

EXPOSE 8080

CMD ["monadium"]