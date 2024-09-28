#!/usr/bin/env bash
# This script is used to test a clean install and tests for
# the built wheel images.
# Steps:
# 1. Run `poetry run maturin build --release` to build the wheel
# 2. Build the docker image to copy `target/wheels/*.whl` to the image

# Build the wheel
poetry run maturin build --release \
  && \
  docker build -t alea-preprocess-ubuntu2404-install -f docker/ubuntu2404-install/Dockerfile . \
  && \
  docker run alea-preprocess-ubuntu2404-install
