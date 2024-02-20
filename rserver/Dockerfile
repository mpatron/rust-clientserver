FROM rust:slim as builder
WORKDIR /usr/src/myapp
COPY . .
RUN cargo install --verbose --path .

FROM ubuntu:22.04
# RUN apt-get update && apt-get install -y extra-runtime-dependencies && rm -rf /var/lib/apt/lists/*
RUN apt-get update && apt-get install -y ca-certificates && rm -rf /var/lib/apt/lists/*

# FROM alpine:3.19.1
# RUN apk add --no-cache ca-certificates
COPY --from=builder /usr/src/myapp/target/release/rserver /usr/local/bin/rserver
EXPOSE 3333
CMD [ "/usr/local/bin/rserver" ]
# CMD ["/bin/sleep 3650d"]