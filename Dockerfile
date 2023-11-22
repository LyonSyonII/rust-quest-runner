FROM rust:slim

WORKDIR /usr/src/rust-quest-runner
COPY . .
RUN cargo install --path . && cargo clean

ENV PORT="3030"
ENV AUTH=""
ENV ORIGINS_WHITELIST=""
ENV SEMAPHORE_PERMITS=5
ENV SEMAPHORE_WAIT=500
ENV KILL_TIMEOUT=500

EXPOSE ${PORT}

ENTRYPOINT ["runner"]