FROM rust:1.59
FROM ubuntu:18.04
FROM ros:foxy

RUN apt update
RUN apt install cargo -y

WORKDIR /docker-rust-client
CMD ["bash", "compilerun.sh"]