FROM scratch
ADD target/x86_64-alpine-linux-musl/release/hyper-tiny-server /hyper-tiny-server
EXPOSE 8080
CMD ["/hyper-tiny-server"]