FROM debian:buster-slim

WORKDIR /app

# RUN apt-get update && apt-get install -y \
#    tini \
    # libssl1.1 \
    # libcurl4 \
    # ;

# RUN useradd app

RUN apt-get update && apt-get -y install libpq-dev && rm -rf /var/lib/apt/lists/* 

COPY /target/release/app /usr/local/bin

# Test if the binary is executable in this environment. This ensures it's built
# for the correct architecture and all shared libraries are available.
# --selfcheck-only means the binary will immediately exit.
# RUN /workspace_service --selfcheck-only

# RUN chown -R app /app

# USER app

EXPOSE 8080

CMD ["app"]