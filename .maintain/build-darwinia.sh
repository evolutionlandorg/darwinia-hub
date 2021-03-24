#!/bin/sh
#
#
#

set -x

BIN_PATH=$(dirname $(readlink -f $0))
WORK_PATH=${BIN_PATH}/../

ls -lah ${WORK_PATH}/target

ARCH=$1

IMAGE_RUST=

if [ "$ARCH" = "x86_64" ]; then
  IMAGE_RUST=rust:1
fi
if [ "$ARCH" = "aarch64" ]; then
  IMAGE_RUST=rustembedded/cross:aarch64-unknown-linux-gnu
fi

docker pull ${IMAGE_RUST}

docker run --rm -i \
  -v ${WORK_PATH}:/data/darwinia \
  -e RUST_TOOLCHAIN=${RUST_TOOLCHAIN} \
  -e CARGO_TERM_COLOR=always \
  -e ARCH=${ARCH} \
  ${IMAGE_RUST} \
  sh -f /data/darwinia/.maintain/docker/_build-darwinia.sh

mkdir -p ${WORK_PATH}/bin


if [ "$ARCH" = "x86_64" ]; then
  cp ${WORK_PATH}/target/release/darwinia ${WORK_PATH}/bin/darwinia_x86_64-linux-gnu
fi
if [ "$ARCH" = "aarch64" ]; then
  cp ${WORK_PATH}/target/release/darwinia ${WORK_PATH}/bin/darwinia_aarch64-linux-gnu
fi

