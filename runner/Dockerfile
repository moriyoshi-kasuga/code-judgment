FROM rust:1.87.0-alpine3.21 AS builder

COPY . /app
WORKDIR /app

RUN apk add --no-cache alpine-sdk musl-dev 

RUN --mount=type=cache,target=/app/target \
  --mount=type=cache,target=/usr/local/cargo/git/db \
  --mount=type=cache,target=/usr/local/cargo/registry \
  cargo build --release --bin runner && \
  mkdir /build && \
  cp /app/target/release/runner /build/runner && \
  cp /app/runner/build.sh /build/build.sh

FROM nixos/nix:2.28.3 AS runner

COPY runner/docker.nix /default.nix
RUN mkdir /runner /running

COPY --from=builder /build/runner /runner/runner
COPY --from=builder /build/build.sh /build.sh

RUN chmod +x /build.sh && /build.sh

RUN nix-env -iA nixpkgs.time nixpkgs.nsjail

ENTRYPOINT [ "/runner/runner" ]

# # for testing
# RUN nix-env -iA nixpkgs.vim

# # this is sample code. todo remove
# nsjail -Mo --chroot / --cgroup_mem_max 104 -- /root/.nix-profile/bin/echo "hey"
# nsjail -Mo --chroot / --cgroup_mem_max 104 --use_cgroupv2 -- /root/.nix-profile/bin/echo "hey"
