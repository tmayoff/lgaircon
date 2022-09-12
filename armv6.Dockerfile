# base pre-built cross image
FROM ghcr.io/cross-rs/arm-unknown-linux-gnueabihf:edge

ENV DEBIAN_FRONTEND noninteractive

# add our foreign architecture and install our dependencies
RUN apt-get update && apt-get install -y --no-install-recommends apt-utils
RUN dpkg --add-architecture armhf
RUN apt-get update && apt-get -y install git xsltproc python3 tclsh

ENV CC arm-unknown-linux-gnueabihf-gcc

# Install LIRC from source
RUN git clone git://git.code.sf.net/p/lirc/git lirc
WORKDIR lirc
RUN ./autogen.sh && ./configure --host arm-unknown-linux-gnueabihf 
RUN make -j`nproc`
RUN make install

WORKDIR ..
RUN git clone --branch version-3.39.3 https://github.com/sqlite/sqlite.git
RUN mkdir sqlite-build
WORKDIR sqlite-build
RUN ../sqlite/configure --host arm-unknown-linux-gnueabihf
RUN make 
RUN make sqlite3.c
RUN make install

ENV CARGO_TARGET_ARM_UNKNOWN_LINUX_GNUEABIHF_RUSTFLAGS="-L /usr/local/lib -L /lib/arm-linux-gnueabihf -C link-args=-Wl,-rpath-link,/usr/lib/arm-linux-gnueabihf $CARGO_TARGET_ARM_UNKNOWN_LINUX_GNUEABIHF_RUSTFLAGS"