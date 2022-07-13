# See here for image contents: https://github.com/microsoft/vscode-dev-containers/tree/v0.238.1/containers/rust/.devcontainer/base.Dockerfile

# [Choice] Debian OS version (use bullseye on local arm64/Apple Silicon): buster, bullseye
ARG VARIANT="bullseye"
FROM mcr.microsoft.com/vscode/devcontainers/rust:0-${VARIANT}

RUN rustup self update

# [Optional] Uncomment this section to install additional packages.
RUN apt-get update && export DEBIAN_FRONTEND=noninteractive \
	&& apt-get -y install --no-install-recommends \
	software-properties-common apt-transport-https wget ca-certificates gnupg2

RUN wget -O- http://repo.mysql.com/RPM-GPG-KEY-mysql-2022 \
	| gpg --dearmor \
	| tee /usr/share/keyrings/mysql.gpg

RUN echo 'deb [signed-by=/usr/share/keyrings/mysql.gpg] http://repo.mysql.com/apt/debian bullseye mysql-8.0' \
	| tee /etc/apt/sources.list.d/mysql.list

RUN apt-get update && export DEBIAN_FRONTEND=noninteractive \
	&& apt-get -y install --no-install-recommends mysql-client

RUN wget https://github.com/planetscale/cli/releases/download/v0.110.0/pscale_0.110.0_linux_amd64.deb \
	&& dpkg -i pscale_* \
	&& rm pscale_*

USER vscode

WORKDIR /home/vscode

RUN curl https://get.volta.sh | bash

RUN /home/vscode/.volta/bin/volta install node && /home/vscode/.volta/bin/volta install yarn
