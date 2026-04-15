FROM debian:bookworm-slim
WORKDIR /app

RUN apt-get update \
    && apt-get install -y --no-install-recommends ca-certificates \
    && rm -rf /var/lib/apt/lists/*

COPY target/release/aiinvite /usr/local/bin/aiinvite

EXPOSE 8080

CMD ["aiinvite", "serve"]
