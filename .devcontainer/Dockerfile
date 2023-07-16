FROM mcr.microsoft.com/devcontainers/rust:0-1-bullseye

RUN apt-get update
RUN apt-get -y install software-properties-common

RUN wget -O - https://apt.llvm.org/llvm-snapshot.gpg.key | apt-key add -
RUN add-apt-repository 'deb http://apt.llvm.org/bullseye/ llvm-toolchain-bullseye-15 main'
RUN apt-get update
RUN apt-get -y install llvm-15 libpolly-15-dev clang-15
RUN ln -s /usr/lib/x86_64-linux-gnu/libclang-15.so.1 /usr/lib/x86_64-linux-gnu/libclang-15.so
RUN ln -s /usr/bin/llvm-config-15 /usr/bin/llvm-config
