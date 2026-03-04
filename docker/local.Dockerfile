FROM rust:1-bookworm

RUN apt-get update \
    && apt-get install -y --no-install-recommends \
        ca-certificates \
        cmake \
        build-essential \
        pkg-config \
    && rm -rf /var/lib/apt/lists/*

COPY docker/scripts/local_entry.sh /usr/local/bin/local_entry.sh
RUN chmod +x /usr/local/bin/local_entry.sh

WORKDIR /work
ENTRYPOINT ["/usr/local/bin/local_entry.sh"]
