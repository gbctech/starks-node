# Note: We don't use Alpine and its packaged Rust/Cargo because they're too often out of date,
# preventing them from being used to build Substrate/Polkadot.

FROM phusion/baseimage:0.11 as builder
LABEL maintainer="zhangx@gbctech.cn"
LABEL description="This is the build stage for Starks Node. Here we create the binary."

ENV DEBIAN_FRONTEND=noninteractive

ARG PROFILE=release
WORKDIR /starks-node

COPY . /starks-node

RUN apt-get update && \
	apt-get dist-upgrade -y -o Dpkg::Options::="--force-confold" && \
	apt-get install -y cmake pkg-config libssl-dev git clang

RUN curl https://sh.rustup.rs -sSf | sh -s -- -y && \
	export PATH="$PATH:$HOME/.cargo/bin" && \
	rustup toolchain install nightly-2020-10-06 && \
	rustup target add wasm32-unknown-unknown --toolchain nightly-2020-10-06-x86_64-unknown-linux-gnu && \
	rustup default stable && \
	cargo build "--$PROFILE"

# ===== SECOND STAGE ======

FROM phusion/baseimage:0.11
LABEL maintainer="zhangx@gbctech.cn"
LABEL description="This is the 2nd stage: a very small image where we copy the Starks Node binary."
ARG PROFILE=release

RUN mv /usr/share/ca* /tmp && \
	rm -rf /usr/share/*  && \
	mv /tmp/ca-certificates /usr/share/ && \
	useradd -m -u 1000 -U -s /bin/sh -d /starks-node starks-node

COPY --from=builder /starks-node/target/$PROFILE/starks-node /usr/local/bin

# checks
RUN ldd /usr/local/bin/starks-node && \
	/usr/local/bin/starks-node --version

# Shrinking
RUN rm -rf /usr/lib/python* && \
	rm -rf /usr/bin /usr/sbin /usr/share/man

USER starks-node
EXPOSE 30333 9933 9944 9615

RUN mkdir /starks-node/data

VOLUME ["/starks-node/data"]

ENTRYPOINT ["/usr/local/bin/starks-node"]