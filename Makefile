CURRENT_DIR = $(shell pwd)
.PHONY: run

run:
	@echo "Running..."
	@LD_LIBRARY_PATH=$(CURRENT_DIR)/deps/depthai-core/build/install/lib cargo run

.PHONY: build-depthai
build-depthai:
	@echo "Building depthai-core..."
	@cd deps/depthai-core && \
		mkdir -p build && \
		cmake -S. -Bbuild -D'BUILD_SHARED_LIBS=ON' -D'DEPTHAI_BUILD_EXAMPLES=ON' && \
		cmake --build build --parallel 4 && \
		cmake --build build --target install