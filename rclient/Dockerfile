FROM rust:slim
WORKDIR /usr/src/myapp
COPY . .
# => /usr/src/myapp/target/release/rclient
RUN cargo install --path .
# CMD ["myapp"]
CMD ["/usr/src/myapp/target/release/rclient"]
# CMD ["/bin/sleep 3650d"]