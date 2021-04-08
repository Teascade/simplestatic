FROM rust:1.51 as builder
WORKDIR /usr/src/simplestatic
COPY . .
RUN cargo install --path .

FROM debian:buster-slim
# RUN apt-get update && apt-get install -y extra-runtime-dependencies && rm -rf /var/lib/apt/lists/*
COPY --from=builder /usr/local/cargo/bin/simplestatic /usr/local/bin/simplestatic
CMD ["simplestatic"]