FROM rust:latest AS builder

WORKDIR /agent

COPY . .

RUN cargo build --release

FROM debian:bookworm-slim

RUN apt-get update && apt-get install -y ca-certificates libssl3 && rm -rf /var/lib/apt/lists/*
RUN curl -fsSL https://ollama.ai/install.sh | sh

WORKDIR /agent

COPY --from=builder /agent/target/release/agent ./

CMD ["./agent", "--mode", "agent"]
