FROM debian:bookworm
RUN apt-get update -y && apt-get upgrade -y && apt-get install -y curl wget vim tmux build-essential pkg-config
RUN curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y
RUN apt-get install nodejs npm -y
RUN npm install --global yarn 
ENV PATH="/root/.cargo/bin:${PATH}"
RUN rustc --version && npm --version && yarn --version
RUN sh -c "$(curl -sSfL https://release.anza.xyz/stable/install)"
ENV PATH="/root/.local/share/solana/install/active_release/bin:$PATH"
RUN solana --version
RUN cargo install --git https://github.com/coral-xyz/anchor avm --force
RUN avm --version
RUN avm install latest && avm use latest
RUN anchor --version

RUN apt-get install -y libwebkit2gtk-4.1-dev \
build-essential \
curl \
wget \
file \
libxdo-dev \
libssl-dev \
libayatana-appindicator3-dev \
librsvg2-dev

RUN apt-get install -y pkg-config libssl-dev cmake
RUN curl -L --proto '=https' --tlsv1.2 -sSf https://raw.githubusercontent.com/cargo-bins/cargo-binstall/main/install-from-binstall-release.sh | bash
RUN npm install --global tailwindcss
RUN cargo binstall dioxus-cli


RUN mkdir -p /docker
ADD entrypoint.sh /docker/entrypoint.sh
RUN chmod +x /docker/entrypoint.sh
ENTRYPOINT ["/docker/entrypoint.sh"]
WORKDIR /docker
EXPOSE 8899

RUN mkdir -p /docker/node_modules /docker/cargo_target
# ENV NPM_CONFIG_PREFIX=/docker/node_modules
# ENV CARGO_TARGET_DIR=/docker/cargo_target

RUN mkdir -p /docker/validator
RUN cd /docker/validator && nohup bash -c \
    "set -ex; (  while true; do solana-test-validator  || sleep 3; done ) & \
     ( sleep 5 \
     && solana config set --url localhost \
     && solana config get  \
     &&  solana-keygen new --no-bip39-passphrase --force \
     && solana address \
     && until solana airdrop 666.666666; do sleep 3; done \
     && solana balance \
     && until solana ping --count 1; do sleep 3; done  \
     )"

RUN mkdir -p /app
WORKDIR /app