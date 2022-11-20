# Using DepthAI Core from Rust

The goal of the project is to make a showcase how to use C++ written library from Rust application.
The project uses [cxx crate](https://lib.rs/crates/cxx) to build the C++ <-> Rust bridge.
This is example from [DepthAI Examples for encoding rgb to h265](https://github.com/luxonis/depthai-core/blob/main/examples/VideoEncoder/rgb_encoding.cpp) - the example uses [DepthAI Core Library](https://github.com/luxonis/depthai-core) and records/writes H265 stream to the disk.


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

## Blog post

[Using C++ library From Rust - DepthAI RGB Camera Example](https://npenkov.com/post/2022/11/19/using-cpp-from-rust-depthai-core-rgb-camera-example/)

or:

[on pnkv.dev domain](https://pnkv.dev/post/2022/11/19/using-cpp-from-rust-depthai-core-rgb-camera-example/)