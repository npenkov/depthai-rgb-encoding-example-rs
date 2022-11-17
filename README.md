# Using Depthai from Rust

## Installation

```
--recurse-submodules
```

Clone repository
Checkout submodules (depthai is in deps/depthai-core)
```
git submodule update --init --recursive
```

Build and install depthai-core

```
cd deps/depthai-core
mkdir -p build
cmake -S. -Bbuild -D'BUILD_SHARED_LIBS=ON' -D'DEPTHAI_BUILD_EXAMPLES=ON'
cmake --build build --parallel 4
cmake --build build --target install
```

