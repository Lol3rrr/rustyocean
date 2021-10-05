FROM rust:1.55 as builder

RUN USER=root cargo new --bin rustyocean
WORKDIR ./rustyocean

COPY . ./

RUN cargo build --release

FROM debian:buster-slim
ARG APP=/usr/src/app

RUN apt-get update; apt-get upgrade -y; apt-get install libssl1.1 ca-certificates -y

RUN mkdir -p ${APP}

COPY --from=builder /rustyocean/target/release/rustyocean ${APP}/rustyocean

WORKDIR ${APP}

ENTRYPOINT ["./rustyocean"]
