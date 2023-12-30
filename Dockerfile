FROM rust:1.74-bookworm as builder
WORKDIR /usr/src/vkgates
COPY . .
RUN --mount=type=cache,target=/usr/src/vkgates/target \
   cargo build --release

FROM scratch as binaries
COPY --from=builder /usr/src/vkgates/target/release/vkgates /

FROM debian:bookworm as server
RUN apt-get update && apt-get install -y libssl3 && rm -rf /var/lib/apt/lists/*
COPY --from=builder /usr/src/vkgates/target/release/vkgates /usr/local/bin/vkgates
CMD ["vkgates"]