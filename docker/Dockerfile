FROM debian:bookworm
RUN apt-get update -y && apt-get upgrade -y \
&& apt-get install -y \
curl wget vim tmux build-essential pkg-config  libwebkit2gtk-4.1-dev \
build-essential \
curl \
wget \
file \
libxdo-dev \
libssl-dev \
libayatana-appindicator3-dev \
librsvg2-dev nodejs npm cmake git

RUN curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y

ENV PATH="/root/.cargo/bin:${PATH}"
ENV RUSTUP_TOOLCHAIN="nightly-2024-11-19"
RUN rustup default nightly-2024-11-19

RUN npm install --global yarn 
RUN npm install --global tailwindcss
RUN npm install --global pnpm@9.10.0

RUN rustc --version && npm --version && yarn --version

# INSTALL SOLANA EXPLORER - DOES NOT BUILD IN PRODUCTION MODE WITHOUT DOING DEV FIRST!
ENV NEXT_TELEMETRY_DISABLED=1
RUN mkdir -p /solana_explorer \
 && cd /solana_explorer \
 && git clone https://github.com/solana-labs/explorer . \
 && git  reset --hard  96704a80808ef0a409669da71ebc2782abf8cda8 \
 && echo "NEXT_PUBLIC_MAINNET_RPC_URL=http://localhost:8899" > .env \
 && echo "NEXT_PUBLIC_DEVNET_RPC_URL=http://localhost:8899" >> .env \
 && echo "NEXT_PUBLIC_TESTNET_RPC_URL=http://localhost:8899" >> .env \
 && pnpm i \
 && pnpm exec next telemetry disable \
 && pnpm run build
ENV NODE_ENV=production
RUN cd /solana_explorer && pnpm run build && ( timeout -k 3s -s 9 2s pnpm start || true )

# INSTALL SOLANA
RUN sh -c "$(curl -sSfL https://release.anza.xyz/v2.1.6/install)"
ENV PATH="/root/.local/share/solana/install/active_release/bin:$PATH"
RUN solana --version

# INSTALL ANCHOR - OFF, see https://github.com/coral-xyz/anchor/issues/3460
# RUN cargo install --git https://github.com/coral-xyz/anchor avm --force
# RUN avm --version
# RUN avm install latest && avm use latest
# RUN anchor --version

# INSTALL BINSTALL + DIOXUS
RUN curl -L --proto '=https' --tlsv1.2 -sSf https://raw.githubusercontent.com/cargo-bins/cargo-binstall/main/install-from-binstall-release.sh | bash
RUN cargo binstall dioxus-cli


# INSTALL SCCACHE
# RUN set -ex && mkdir -p /sccache && cd /sccache \
# && wget https://github.com/mozilla/sccache/releases/download/v0.9.0/sccache-v0.9.0-x86_64-unknown-linux-musl.tar.gz \
# && tar -xzvf sccache-v0.9.0-x86_64-unknown-linux-musl.tar.gz \
# && mv sccache-v0.9.0-x86_64-unknown-linux-musl/sccache .
# ENV PATH="/sccache:$PATH"
# RUN sccache --version && sccache --start-server
# DONT ENABLE SCCACHE => cargo-test-sbf causes error over mounted target
# ENV RUSTC_WRAPPER=/sccache/sccache


# INSTALL ENTRYPOINT
ENTRYPOINT ["/bin/bash", "-ex", "/app/docker/entrypoint.sh"]
RUN mkdir -p /docker
WORKDIR /docker
EXPOSE 8899


# run anchor validator and framework
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

WORKDIR /app


# DOES NOT FUCKING WORK MOVING THESE, ANCHOR IS HARD-CODED
RUN mkdir -p /docker/cargo_target
# ENV NPM_CONFIG_PREFIX=/docker/node_modules
ENV CARGO_TARGET_DIR=/docker/cargo_target

ADD pacanele2 /docker/pacanele2_build_init
RUN cd /docker/pacanele2_build_init && cargo build-sbf && cargo test-sbf 
RUN bash -c \
"set -ex; ( cd /docker/validator ; while true; do solana-test-validator  || sleep 3; done ) & \
 ( sleep 5 \
    && until solana ping --count 1; do sleep 3; done \
   && ( cd /docker/pacanele2_build_init && solana program deploy ${CARGO_TARGET_DIR}/deploy/pacanele2.so ) )"

VOLUME /docker/cargo_target