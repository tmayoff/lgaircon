# base pre-built cross image
FROM ghcr.io/cross-rs/arm-unknown-linux-gnueabihf:edge

ENV DEBIAN_FRONTEND noninteractive

# add our foreign architecture and install our dependencies
RUN apt-get update && apt-get install -y --no-install-recommends apt-utils
RUN dpkg --add-architecture armhf
RUN apt-get update && apt-get -y install libsqlite3-dev:armhf liblircclient-dev:armhf
