# Earthly.dev build environment for Rust on ESP32

## Overview

This repo provides an isolated and reproducible build environment for creating Rust binaries that run on the ESP32.   
A simple "hello world" project is present in [main.rs](src/main.rs).

## Prerequisites
 - Docker
 - [earthly](https://earthly.dev/), installed 
 - [espfash](https://github.com/esp-rs/espflash) cargo tool, used to flash the esp32

## How to use

 1. Clone this repo
 2. Put your code under `src`
 3. Update [Cargo.toml](Cargo.toml)
 4. Run either the one of the following commands to build the firmware
 Debug build:

 ```
 earthly +firmware
 ```
Release build:
```
earthly --build-arg BUILD_TYPE=release  +firmware
```

5. Flash the firmware
Where: `/dev/ttyUSB0` is the serial device of your ESP32.   
Where: `esp32-rust-build-environment` is the project name from your `Cargo.toml` file.

```
earthly --build-arg SERIAL_PORT=/dev/ttyUSB0 --build-arg BUILD_BINARY=artifacts/debug/esp32-rust-build-environment +flash
```
