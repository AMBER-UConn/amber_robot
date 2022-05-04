FROM rust:1.59

FROM ros:noetic

RUN apt update
RUN apt install cargo -y

WORKDIR /docker-rust-client
CMD ["bash", "compilerun.sh"]