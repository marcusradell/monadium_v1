FROM rust as builder
WORKDIR /app
COPY . .
RUN cargo build --release --bin app

FROM rust:slim as runtime
WORKDIR /app
RUN apt-get update && apt-get -y install libpq-dev && rm -rf /var/lib/apt/lists/* 
COPY --from=builder /app/target/release/app /usr/local/bin
EXPOSE 8080
CMD ["app"]