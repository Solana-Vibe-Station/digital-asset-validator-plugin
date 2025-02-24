ARG SOLANA_VERSION=v2.1.11
ARG RUST_VERSION=1.83.0

##
# 1. Build stage
##
FROM rust:${RUST_VERSION}-bullseye as builder

RUN apt-get update \
 && apt-get -y install \
    wget \
    curl \
    build-essential \
    software-properties-common \
    lsb-release \
    libelf-dev \
    linux-headers-generic \
    pkg-config \
    curl \
    cmake \
    protobuf-compiler

RUN curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y
ENV PATH="/root/.cargo/bin:${PATH}"

WORKDIR /rust/
COPY plerkle_serialization /rust/plerkle_serialization
COPY plerkle_messenger       /rust/plerkle_messenger
COPY plerkle_snapshot        /rust/plerkle_snapshot
COPY plerkle                 /rust/plerkle
COPY Cargo.toml              /rust/
COPY Cargo.lock              /rust/
WORKDIR /rust

RUN cargo build --release

##
# 2. Final stage (Solana container)
##
FROM anzaxyz/agave:${SOLANA_VERSION}

# Install wget (and dpkg if needed) so we can download & install libssl
RUN apt-get update && apt-get install -y wget libssl-dev dpkg

# Example: install libssl1.1 from a .deb file
RUN wget http://nz2.archive.ubuntu.com/ubuntu/pool/main/o/openssl/libssl1.1_1.1.1f-1ubuntu2.24_amd64.deb \
 && dpkg -i libssl1.1_1.1.1f-1ubuntu2.24_amd64.deb \
 && apt-get -fy install

# Copy plugin from build stage
COPY --from=builder /rust/target/release/libplerkle.so /plugin/plugin.so

# Copy shell scripts from local docker folder
COPY ./docker .
RUN chmod +x ./*.sh

ENTRYPOINT ["./runs.sh"]
CMD [""]
