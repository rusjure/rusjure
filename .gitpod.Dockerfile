FROM debian:bullseye

RUN sudo apt-get update
RUN sudo apt-get -y install software-properties-common apt-transport-https

RUN sudo sh -c "wget -O - https://apt.llvm.org/llvm-snapshot.gpg.key | apt-key add -"
RUN sudo add-apt-repository 'deb http://apt.llvm.org/bullseye/ llvm-toolchain-bullseye-15 main'
RUN sudo apt-get update
RUN sudo apt-get -y install llvm-15 libpolly-15-dev

RUN sudo curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y
