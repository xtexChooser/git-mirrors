FROM docker.io/library/alpine as builder
RUN apk add -q --no-cache build-base openssl-dev curl
RUN curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y --default-toolchain nightly --profile minimal

WORKDIR /src
COPY spica-signer spica-signer
COPY spica-signer-common spica-signer-common
COPY ["Cargo*", "LICENSE*", "./"]
ENV RUSTFLAGS="-C target-feature=-crt-static"

RUN source "$HOME/.cargo/env"; cargo install --path=spica-signer --root=/app

FROM docker.io/library/alpine
RUN apk add libgcc
COPY --from=builder /app /
