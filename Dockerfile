FROM rust:1-bookworm AS builder

WORKDIR /app

COPY Cargo.toml .
COPY Cargo.lock .
COPY src src

RUN cargo build --release

FROM python:3.12-slim-bookworm

COPY install-packages.sh .
RUN ./install-packages.sh

RUN python -m pip install auto-editor yt-dlp

COPY --from=builder /app/target/release/auto-editor-docker /wrapper

ENTRYPOINT [ "/wrapper" ]
