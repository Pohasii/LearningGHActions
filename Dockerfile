####################################################################################################
## Builder
####################################################################################################
FROM rust:1.54.0-alpine3.13 AS builder

WORKDIR /app

COPY ./ .

# We no longer need to use the x86_64-unknown-linux-musl target
RUN cargo build --release

####################################################################################################
## Final image
####################################################################################################
FROM alpine:latest
# debian:buster-slim

WORKDIR /app

# Copy our build
COPY --from=builder /app/target/release/fastUdpSocket ./

EXPOSE 55442/udp

CMD ["./fastUdpSocket"]