FROM rust:1.76-slim as builder
WORKDIR /usr/src/myapp
COPY . .
RUN cargo install --verbose --path .

FROM ubuntu:22.04
RUN apt-get update && apt-get install -y ca-certificates ncat && rm -rf /var/lib/apt/lists/*

COPY --from=builder /usr/src/myapp/target/release/rclient /usr/local/bin/rclient
EXPOSE 3333
CMD [ "/usr/local/bin/rclient" ]
