FROM rust:1.89.0-slim AS builder

RUN apt-get update && \
    apt-get install -y --no-install-recommends \
        musl-tools \
        musl-dev \
        gcc && \
    rm -rf /var/lib/apt/lists/*

RUN rustup target add x86_64-unknown-linux-musl

WORKDIR /usr/src/app

COPY . .

RUN cargo build --release --target x86_64-unknown-linux-musl

FROM scratch

COPY --from=builder /usr/src/app/target/x86_64-unknown-linux-musl/release/taskrunner /taskrunner

ENTRYPOINT ["/taskrunner"]
