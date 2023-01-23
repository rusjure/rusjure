FROM mcr.microsoft.com/devcontainers/rust:0-1-bullseye

RUN apt-get update
RUN apt-get -y install software-properties-common

RUN wget -O - https://apt.llvm.org/llvm-snapshot.gpg.key | apt-key add -
RUN add-apt-repository 'deb http://apt.llvm.org/bullseye/ llvm-toolchain-bullseye-14 main'
RUN apt-get update
RUN apt-get -y install llvm-14 libpolly-14-dev

# RUN wget https://apt.llvm.org/llvm.sh
# RUN chmod +x llvm.sh
# RUN bash ./llvm.sh 14 all
