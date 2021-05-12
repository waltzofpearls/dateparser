FROM rust:1.50.0-buster

ENV CROSS_DOCKER_IN_DOCKER=true
RUN cargo install cross

RUN apt-get update \
 && apt-get install -y --no-install-recommends \
        apt-transport-https \
        ca-certificates \
        curl \
        gnupg \
        lsb-release

RUN curl -fsSL https://download.docker.com/linux/debian/gpg | gpg --dearmor -o /usr/share/keyrings/docker-archive-keyring.gpg
RUN echo \
  "deb [arch=amd64 signed-by=/usr/share/keyrings/docker-archive-keyring.gpg] https://download.docker.com/linux/debian \
  $(lsb_release -cs) stable" | tee /etc/apt/sources.list.d/docker.list > /dev/null

RUN apt-get update \
 && apt-get install -y --no-install-recommends \
        docker-ce \
        docker-ce-cli \
        containerd.io
