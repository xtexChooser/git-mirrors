FROM docker.io/library/rust:alpine AS builder
RUN rustup default nightly
RUN apk add openssl-dev musl-dev
ENV RUSTFLAGS "-C target-feature=-crt-static"
RUN cargo install dn42-kb-canvas-wikipedia-rc --root=/app
RUN ls -R -l /app

FROM docker.io/library/alpine
RUN apk add libgcc
ENV KB_CANVAS_PREFIX fdcf:8538:9ad5:3333
COPY --from=builder /app /
ENTRYPOINT [ "dn42-kb-canvas-wikipedia-rc" ]
