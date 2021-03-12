FROM rust:1.50.0 as planner
WORKDIR /app
# We only pay the installation cost once, 
# it will be cached from the second build onwards
# To ensure a reproducible build consider pinning 
# the cargo-chef version with `--version X.X.X`
RUN cargo install cargo-chef 
COPY . .
RUN cargo chef prepare  --recipe-path recipe.json

FROM rust:1.50.0 as cacher
WORKDIR /app
RUN cargo install cargo-chef
COPY --from=planner /app/recipe.json recipe.json
RUN cargo chef cook --release --recipe-path recipe.json

FROM rust:1.50.0 as builder
WORKDIR /app
COPY . .
# Copy over the cached dependencies
COPY --from=cacher /app/target target
COPY --from=cacher /usr/local/cargo /usr/local/cargo
RUN cargo build --release --bin app

FROM rust:1.50.0-slim as runtime
WORKDIR /app
RUN apt-get update && apt-get -y install libpq-dev && rm -rf /var/lib/apt/lists/* 
COPY --from=builder /app/target/release/app /usr/local/bin

EXPOSE 8080
CMD ["app"]