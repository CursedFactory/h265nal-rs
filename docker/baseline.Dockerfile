FROM debian:bookworm-slim

RUN apt-get update \
    && apt-get install -y --no-install-recommends \
        ca-certificates \
        cmake \
        build-essential \
        git \
    && rm -rf /var/lib/apt/lists/*

COPY docker/scripts/baseline_entry.sh /usr/local/bin/baseline_entry.sh
RUN chmod +x /usr/local/bin/baseline_entry.sh

WORKDIR /work
ENTRYPOINT ["/usr/local/bin/baseline_entry.sh"]
