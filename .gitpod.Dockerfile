FROM debian:bullseye

USER root

RUN apt-get update
RUN apt-get -y install software-properties-common apt-transport-https

RUN add-apt-repository 'deb http://apt.llvm.org/bullseye/ llvm-toolchain-bullseye-15 main'
RUN apt-get update
RUN apt-get -y install llvm-15 libpolly-15-dev

RUN curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y
