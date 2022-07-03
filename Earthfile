# This Earthfile aims to build Rust for the ESP32 microcontrollers.
# Makes use of the well tested scripted install from the Rust ESP comunity:
# https://github.com/esp-rs/rust-build

FROM debian:stable-slim

linux-setup:
  RUN apt update && \
      DEBIAN_FRONTEND=noninteractive apt install -y \
      git curl gcc ninja-build cmake libudev-dev \
      python3 python3-pip libusb-1.0-0 libssl-dev pkg-config libtinfo5 unzip

esp-rust-setup:
  FROM +linux-setup
  RUN git clone https://github.com/esp-rs/rust-build.git
  RUN cd rust-build && ./install-rust-toolchain.sh --extra-crates "ldproxy"

firmware:
  FROM +esp-rust-setup
  WORKDIR build
  
  # Build type, default is debug. 
  # can be overridden on the command line via "--build-arg BUILD_TYPE=release" to get a release build.
  ARG BUILD_TYPE
  
  # These vaules are emited after installing the toolchain.
  # There is no way yet to get this progamaticly, thus this will need updating at times when build tooling updates.
  ENV PATH="/root/.cargo/bin:/root/.espressif/tools/xtensa-esp32-elf-clang/esp-14.0.0-20220415-x86_64-unknown-linux-gnu/bin/:$PATH"
  ENV LIBCLANG_PATH="/root/.espressif/tools/xtensa-esp32-elf-clang/esp-14.0.0-20220415-x86_64-unknown-linux-gnu/lib/"

  COPY . .
  # Set Rust to use ESP as the default toolchain
  RUN rustup default esp
  
  # What type of build, default is debug.
  IF [ "$BUILD_TYPE" = "release" ]
    RUN echo "[INFO] Building a release binary"
    RUN cargo +esp build --target xtensa-esp32-espidf --release
  ELSE
    RUN echo "[INFO] Building a debug binary"
    RUN cargo +esp build --target xtensa-esp32-espidf -v
  END

  SAVE ARTIFACT target/xtensa-esp32-espidf/* AS LOCAL artifacts/

flash:
  LOCALLY
  # This step MUST run in "LOCAL" context as you can not yet pass a USB device into a docker container.
  # You MUST have espflash installed localy:
  # https://github.com/esp-rs/espflash

  # Define build arg for the serial port.
  ARG SERIAL_PORT
  # Pass in the name of the binary to flash, most likey to be "artifacts/debug/rust-earthly-build"
  ARG BUILD_BINARY
  
  # Test if we have espflash installed and in our path.
  RUN echo "This step runs localy and needs your machine to have espflash installed first."
  RUN which espflash 2>/dev/null || echo "[ERROR] espflash not found in path, try install it via: cargo install espflash"
  
  # Flash the firmware
  RUN espflash $SERIAL_PORT $BUILD_BINARY

