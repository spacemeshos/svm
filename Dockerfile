FROM debian:stretch AS svm-build

ENV CARGO_HOME=/usr/local/rust
ENV RUSTUP_HOME=/usr/local/rust
ENV PATH="$PATH:$CARGO_HOME/bin"

RUN apt-get update && \
  apt-get install -y --no-install-recommends cmake curl wget ca-certificates gcc build-essential lsb-release clang \
  && rm -rf /var/lib/apt/lists/* \
  && curl https://sh.rustup.rs -sSf | sh -s -- -y --default-toolchain nightly \
  && curl -SL https://releases.llvm.org/8.0.0/clang+llvm-8.0.0-x86_64-linux-gnu-ubuntu-16.04.tar.xz | tar -xJC /home

ENV LLVM_SYS_80_PREFIX /home/clang+llvm-8.0.0-x86_64-linux-gnu-ubuntu-16.04/

WORKDIR /svm
COPY ["/crates", "/src", "/Cargo.toml", "/Cargo.lock", "/svm/"]
