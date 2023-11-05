# start from a fresh ubuntu image
FROM ubuntu:23.04

# install basic tools
RUN apt-get update && \
    apt-get install -y curl wget bash git nano sudo build-essential

# use non-root user
RUN passwd --delete ubuntu && \
    echo "ubuntu ALL=(ALL:ALL) NOPASSWD: ALL" | tee /etc/sudoers.d/ubuntu
USER ubuntu

# install cool ass looking bash prompt
RUN printf "\nsource /com.docker.devenvironments.code/docker/bash.sh" | tee -a ~/.bashrc

# install rust
RUN curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y