FROM ubuntu:22.04

RUN apt update -y \
  && apt install -y \
   	git \
   	lsb-release \
    build-essential \
    curl \
    libasio-dev \
    libssl-dev \
    libtinyxml2-dev \
  && rm -rf /var/lib/apt/lists/*

RUN curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y --default-toolchain=stable

RUN mkdir /booster_sys
WORKDIR /booster_sys
ENV LD_LIBRARY_PATH="/booster_sys/booster_robotics_sdk/lib/x86_64/:/booster_sys/booster_robotics_sdk/third_party/lib/x86_64/"
