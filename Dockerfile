#
#FROM rust:latest AS builder
#WORKDIR /app
#COPY ./ .
#RUN cargo install --path .
#
#
#FROM alpine:latest
## debian:buster-slim
#WORKDIR /app
#COPY --from=builder /app/target/release/fastUdpSocket ./
#EXPOSE 55442/udp
#CMD ["./fastUdpSocket"]

FROM rust:latest as builder
WORKDIR /app
COPY ./ .
RUN cargo install --path .

FROM debian:buster-slim
#RUN apt-get update && apt-get install -y extra-runtime-dependencies && rm -rf /var/lib/apt/lists/*
WORKDIR /app
COPY --from=builder /app/target/release/fastUdpSocket ./
CMD ["./fastUdpSocket"]