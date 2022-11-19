# Using DepthAI Core from Rust

This is example from [DepthAI Examples for encoding rgb to h265](https://github.com/luxonis/depthai-core/blob/main/examples/VideoEncoder/rgb_encoding.cpp) but written in Rust.
The goal of the project is to make an example how to use C++ written library and to use it from Rust.

## Installation

Clone the repository using recursive for submodules:

```
git clone --recurse-submodules https://github.com/npenkov/depthai-rgb-encoding-example-rs.git
```

or simply clone the reposity and run

```
git submodule update --init --recursive
```

Build and install depthai-core

- Option 1: 
  
  ```
  cd deps/depthai-core
  mkdir -p build
  cmake -S. -Bbuild -D'BUILD_SHARED_LIBS=ON' -D'DEPTHAI_BUILD_EXAMPLES=ON'
  cmake --build build --parallel 4
  cmake --build build --target install
  ```
- Option 2:
  
  ```
  make build-depthai
  ```

## Running the example

```
make run
```