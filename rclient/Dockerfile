FROM rust:slim
WORKDIR /usr/src/myapp
COPY . .
# => /usr/src/myapp/target/release/rclient
RUN cargo install --path .
CMD ["myapp"]