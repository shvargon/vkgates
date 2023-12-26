FROM rust:1.74-bookworm as builder
WORKDIR /usr/src/vkapi
COPY . .
RUN --mount=type=cache,target=/usr/src/vkapi/target \
   cargo build --release

FROM debian:bookworm as server
RUN apt-get update && apt-get install -y libssl3 && rm -rf /var/lib/apt/lists/*
COPY --from=builder /usr/src/vkapi/target/release/vkapi /usr/local/bin/vkapi
CMD ["vkapi"]

FROM scratch as binaries
COPY --from=builder /usr/src/vkapi/target/release/vkapi /