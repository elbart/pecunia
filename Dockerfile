FROM rust:latest as builder

WORKDIR /pecunia

COPY . .
RUN cargo install --path .

FROM debian:buster-slim

RUN apt-get -y update && apt-get -y upgrade && apt-get -y install openssl
COPY --from=builder /usr/local/cargo/bin/pecunia /usr/local/bin/pecunia

CMD ["pecunia"]