FROM rustlang/rust:nightly-slim as builder

RUN USER=root cargo new --bin nomero
WORKDIR /nomero
COPY Cargo.toml Cargo.toml
RUN cargo build --release && rm src/*.rs

COPY . .
RUN rm ./target/release/deps/nomero* && cargo build --release

FROM debian:buster-slim as runtime

ENV TZ=Etc/UTC \
    USER=nomero \
    ROCKET_ENV=prod

WORKDIR /bin

RUN apt-get update && \
    apt-get install -y ca-certificates tzdata && \
    rm -rf /var/lib/apt/lists/* && \
    groupadd ${USER} && \
    useradd -g ${USER} ${USER}

USER ${USER}

COPY --from=builder --chown=${USER}:${USER} /nomero/target/release/nomero ./server

ENTRYPOINT ["/bin/server"]