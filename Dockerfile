#####################################
## 1. Chef
#####################################
FROM rust:1.55 as chef
RUN apt-get update && apt-get install openssl libudev-dev npm -y
RUN cargo install cargo-chef --version 0.1.31
WORKDIR /usr/src/solend-apy-bot

#####################################
## 2. Planner, build recipe
#####################################
FROM chef as planner
COPY . .
RUN cargo chef prepare --recipe-path recipe.json

#####################################
## 2. Cache dependencies
#####################################
FROM chef as cacher
COPY --from=planner /usr/src/solend-apy-bot/recipe.json recipe.json
RUN cargo chef cook --release --recipe-path recipe.json

#####################################
## 3. Build binary
#####################################
FROM chef as builder
# Copy over cached dependencies
COPY --from=cacher /usr/src/solend-apy-bot/target target
COPY --from=cacher /usr/local/cargo /usr/local/cargo
COPY . .
RUN cargo build --release

#####################################
## 4. Runtime environment
#####################################
FROM debian:bullseye-slim as runtime

ENV DEBIAN_FRONTEND=noninteractive
EXPOSE 4444

# Install headless chrome
RUN apt-get update \
    && apt-get install -y wget gnupg2 \
    && wget -q -O - https://dl.google.com/linux/linux_signing_key.pub | apt-key add - \
    && sh -c 'echo "deb [arch=amd64] http://dl.google.com/linux/chrome/deb/ stable main" >> /etc/apt/sources.list.d/google.list' \
    && apt-get update \
    && apt install -fy google-chrome-stable \
    && apt-get update \
    && rm -rf /var/lib/apt/lists/*

COPY --from=builder /usr/src/solend-apy-bot/target/release/solend-apy-bot /usr/local/bin/solend-apy-bot
COPY --from=builder /usr/src/solend-apy-bot/web/dist /usr/src/solend-apy-bot/web/dist

WORKDIR /home/root

ENTRYPOINT [ "solend-apy-bot" ]