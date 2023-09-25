FROM rust:1-slim AS BUILDER

RUN apt update -qq && apt install -y -qq --no-install-recommends \
    gcc \
    musl-tools \
    cmake \
    clang \
    make

COPY ./Cargo.toml /opt/project/
COPY ./src/ /opt/project/src/
COPY ./Cargo.lock /opt/project/

RUN rustup set profile minimal
RUN rustup target add x86_64-unknown-linux-musl

WORKDIR /opt/project
ENV RUSTFLAGS='-C link-args=-s'

RUN cargo build --release --target x86_64-unknown-linux-musl

FROM alpine
RUN apk add --no-cache ca-certificates
COPY --from=BUILDER /opt/project/target/x86_64-unknown-linux-musl/release/tcc-maxtemp-watcher /usr/local/bin/tcc-maxtemp-watcher

RUN chmod a+rx /usr/local/bin/*
RUN adduser tcc-maxtemp-watcher -s /bin/false -D -H
USER tcc-maxtemp-watcher

WORKDIR /usr/local/bin
ENTRYPOINT [ "/usr/local/bin/tcc-maxtemp-watcher" ]