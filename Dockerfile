FROM rust:1.41.1

WORKDIR /usr/src/erc20
COPY . .

RUN apt update
RUN apt -y install cmake protobuf-compiler