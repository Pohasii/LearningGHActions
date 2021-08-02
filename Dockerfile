####################################################################################################
## Builder
####################################################################################################
FROM rust:latest AS builder

WORKDIR /app

COPY ./ .

# We no longer need to use the x86_64-unknown-linux-musl target
RUN cargo build --release

####################################################################################################
## Final image
####################################################################################################
FROM debian:buster-slim

WORKDIR /app

# Copy our build
COPY --from=builder /app/target/release/fastUdpSocket ./

CMD ["./fastUdpSocket"]