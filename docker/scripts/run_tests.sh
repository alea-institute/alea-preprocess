#!/usr/bin/env bash

# change to /app
pushd /app

# source cargo
source $HOME/.cargo/env

# cargo test
cargo test \
  && \
  poetry run maturin develop \
  && \
  poetry run pytest tests/

# change back to original directory
popd
