FROM rust:1.70.0
MAINTAINER cafkafk
WORKDIR /usr/src/eza
COPY . .
# RUN mkdir -p /usr/src/eza/release
RUN cargo build --release
RUN mkdir -p release
CMD ./devtools/dev-release-linux && cp eza-linux-x86_64 release/ && cp eza-linux-x86_64.tar.gz release/
