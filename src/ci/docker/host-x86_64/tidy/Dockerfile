# We use the ghcr base image because ghcr doesn't have a rate limit
# and the tidy job doesn't cache docker images in CI.
FROM ghcr.io/rust-lang/ubuntu:22.04

ARG DEBIAN_FRONTEND=noninteractive
RUN apt-get update && apt-get install -y --no-install-recommends \
  g++ \
  make \
  ninja-build \
  file \
  curl \
  ca-certificates \
  python2.7 \
  python3 \
  python3-pip \
  python3-pkg-resources \
  git \
  cmake \
  sudo \
  gdb \
  xz-utils \
  libssl-dev \
  pkg-config \
  mingw-w64 \
  && rm -rf /var/lib/apt/lists/*

COPY scripts/nodejs.sh /scripts/
RUN sh /scripts/nodejs.sh /node
ENV PATH="/node/bin:${PATH}"

COPY scripts/sccache.sh /scripts/
RUN sh /scripts/sccache.sh

RUN pip3 install virtualenv

# NOTE: intentionally uses python2 for x.py so we can test it still works.
ENV SCRIPT="TIDY_PRINT_DIFF=1 python2.7 ../x.py test \
  src/tools/tidy tidyselftest --extra-checks=py,cpp,js,spellcheck"
