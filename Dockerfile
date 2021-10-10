FROM rust:1.55 as builder

RUN apt-get update && apt-get install git openssl libudev-dev npm -y

WORKDIR /usr/src/solend-apy-bot
COPY . .
#COPY solend-apy-bot /usr/local/cargo/bin/
RUN cargo install --path .

FROM debian:bullseye-slim as executor

ENV DEBIAN_FRONTEND=noninteractive
EXPOSE 4444

RUN apt-get update \
    && apt-get install -y wget gnupg2 \
    && wget -q -O - https://dl.google.com/linux/linux_signing_key.pub | apt-key add - \
    && sh -c 'echo "deb [arch=amd64] http://dl.google.com/linux/chrome/deb/ stable main" >> /etc/apt/sources.list.d/google.list' \
    && apt-get update \
    && apt install -fy google-chrome-stable \
    && apt-get update \
    && rm -rf /var/lib/apt/lists/*

COPY --from=builder /usr/local/cargo/bin/solend-apy-bot /usr/local/bin/solend-apy-bot
COPY --from=builder /usr/src/solend-apy-bot/web/dist /usr/src/solend-apy-bot/web/dist

WORKDIR /home/root

ENTRYPOINT [ "solend-apy-bot" ]