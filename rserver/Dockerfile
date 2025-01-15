FROM docker.io/rust:1.83-slim as builder
WORKDIR /usr/src/myapp
COPY . .
RUN cargo install --verbose --path .

FROM docker.io/ubuntu:24.04
RUN apt-get update && apt-get install -y ca-certificates ncat && rm -rf /var/lib/apt/lists/*
# Ne fonctionne pas encore avec alpine. L'image passerait de 86Mo à 11Mo...
# FROM alpine:3.19.1
# RUN apk add --no-cache ca-certificates netcat-openbsd libgcc libstdc++ libc6-compat

COPY --from=builder /usr/src/myapp/target/release/rserver /usr/local/bin/rserver
EXPOSE 3333
CMD [ "/usr/local/bin/rserver" ]
# CMD ["/bin/sleep 3650d"]