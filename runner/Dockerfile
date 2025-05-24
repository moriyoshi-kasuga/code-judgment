# syntax = devthefuture/dockerfile-x

FROM rust:1.87.0-alpine3.21 AS builder

COPY . /app
WORKDIR /app

RUN apk add --no-cache alpine-sdk musl-dev 

RUN --mount=type=cache,target=/app/target \
  --mount=type=cache,target=/usr/local/cargo/git/db \
  --mount=type=cache,target=/usr/local/cargo/registry \
  cargo build --release --bin runner && \
  mkdir /build && \
  cp /app/target/release/runner /build/runner 

FROM nixos/nix:2.28.3 AS runner

COPY runner/docker.nix /default.nix
RUN nix-env --profile /global -iA nixpkgs.time nixpkgs.nsjail nixpkgs.bash

INCLUDE runner/Dockerfile.build

COPY --from=builder /build/runner /server

ENTRYPOINT [ "/server" ]
