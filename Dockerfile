FROM debian:bullseye-20210311-slim

WORKDIR /app

RUN useradd app

RUN apt-get update && apt-get -y install \
    libpq-dev \
    && rm -rf /var/lib/apt/lists/*

COPY /target/release/app /usr/local/bin

RUN chown -R app /app

USER app

EXPOSE 8080

CMD ["app"]
