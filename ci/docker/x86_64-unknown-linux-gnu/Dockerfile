FROM ubuntu:16.04
RUN apt-get update && \
    apt-get install -y --no-install-recommends \
    ca-certificates \
    curl \
    gcc \
    gcc-arm-none-eabi \
    libc6-dev \
    linkchecker
RUN curl -LSfs https://japaric.github.io/trust/install.sh | \
    sh -s -- --git azerupi/mdbook --tag v0.0.14 --target x86_64-unknown-linux-gnu --to /usr/bin
RUN curl -LSfs https://japaric.github.io/trust/install.sh | \
    sh -s -- --git japaric/xargo --tag v0.3.7 --target x86_64-unknown-linux-gnu --to /usr/bin
