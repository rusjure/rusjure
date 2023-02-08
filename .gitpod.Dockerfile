FROM gitpod/workspace-full

RUN sudo apt-get update
RUN sudo apt-get -y install software-properties-common apt-transport-https wget gnupg git curl

RUN sudo sh -c "wget -O - https://apt.llvm.org/llvm.sh > ./llvm.sh"
RUN sudo chmod a+x ./llvm.sh
RUN sudo ./llvm.sh 15
RUN sudo apt-get -y install libpolly-15-dev
