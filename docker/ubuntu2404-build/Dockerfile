FROM ubuntu:24.04

# Avoid prompts from apt
ENV DEBIAN_FRONTEND=noninteractive

# Install system dependencies
RUN apt-get update && apt-get install -y --no-install-recommends \
    python3 \
    python3-pip \
    python3-venv \
    pipx \
    curl \
    pkg-config \
    cmake \
    libssl-dev \
    build-essential \
    &&  \
    rm -rf /var/lib/apt/lists/* \
    && \
    pipx ensurepath \
    && \
    pipx install poetry \
    && \
    pipx install maturin

# set up rust
RUN curl https://sh.rustup.rs -sSf | bash -s -- -y
RUN echo 'source $HOME/.cargo/env' >> $HOME/.bashrc
RUN bash -l -c "rustup default nightly"

# copy the pdfium install script
COPY docker/scripts/install_pdfium.sh /app/install_pdfium.sh
RUN bash -l -c "bash /app/install_pdfium.sh"

# create /app directory
WORKDIR /app
RUN cd /app

# copy the poetry configuration \
COPY pyproject.toml poetry.lock ./
COPY Cargo.toml Cargo.lock ./

# build the poetry environment
RUN bash -l -c "poetry install --no-root"

# copy the rust code
COPY src src
COPY tests tests
COPY resources resources

# copy the test script
COPY docker/scripts/run_tests.sh /app/run_tests.sh

# run command
CMD ["bash", "-l", "/app/run_tests.sh"]
