FROM ubuntu:24.04

# Avoid prompts from apt
ENV DEBIAN_FRONTEND=noninteractive

# Install system dependencies
RUN apt-get update && apt-get install -y --no-install-recommends \
    python3 \
    python3-pip \
    python3-venv \
    curl

# create /app directory
WORKDIR /app
RUN cd /app

# copy the pdfium install script
COPY docker/scripts/install_pdfium.sh /app/install_pdfium.sh
RUN bash -l -c "bash /app/install_pdfium.sh "

# create a virtual environment
RUN python3 -m venv /app/.venv

# install pytest into it
RUN /app/.venv/bin/pip install pytest

# copy the target/wheels/*.whl files
COPY target/wheels/*.whl /app

# install the wheels
RUN /app/.venv/bin/pip install --no-index /app/*.whl

# copy the tests directory
COPY resources /app/resources
COPY tests /app/tests

# run the tests
CMD ["/app/.venv/bin/pytest", "/app/tests"]
