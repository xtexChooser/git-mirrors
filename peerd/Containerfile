FROM docker.io/library/alpine AS builder

RUN apk add curl musl musl-dev gcc protoc
RUN curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- --default-toolchain nightly -y
ADD src /src/src
ADD Cargo.* /src/
WORKDIR /src
RUN source "$HOME/.cargo/env"; cargo install --path=. --root=/install

FROM docker.io/library/alpine
COPY --from=builder /install /app
WORKDIR /app
ENTRYPOINT [ "/app/bin/peerd" ]
VOLUME [ "/etc/peerd.toml", "/var/run/bird.ctl", "/etc/bird/" ]
