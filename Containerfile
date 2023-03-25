FROM docker.io/library/alpine AS builder

RUN apk add -q --no-cache build-base curl linux-headers openssl-dev libclang
RUN curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y --default-toolchain nightly --profile minimal
WORKDIR /build
COPY src src
COPY ["Cargo*", "LICENSE*", "build.rs", "./"]

ENV RUSTFLAGS="-C target-feature=-crt-static"
RUN source "$HOME/.cargo/env"; cargo install --path=. --root=/app

FROM docker.io/library/alpine
RUN apk add libgcc
COPY --from=builder /app /
WORKDIR /
ENV MEKBUDA_CONFIG = "/etc/mekbuda/mekbuda.toml"
ENTRYPOINT [ "/usr/bin/mekbuda" ]
