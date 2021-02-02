FROM rust:buster as builder
WORKDIR /usr/src/monadium
COPY . .
RUN cargo install --path .

FROM rust:slim-buster
RUN apt-get update && apt-get -y install libpq-dev && rm -rf /var/lib/apt/lists/* 

COPY --from=builder /usr/local/cargo/bin/monadium /usr/local/bin/monadium
EXPOSE 8080
CMD ["monadium"]
