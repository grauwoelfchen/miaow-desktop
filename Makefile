LIBCLANG_PATH := "/usr/lib/llvm/14/lib64"
C_INCLUDE_PATH := "/usr/lib/gcc/x86_64-pc-linux-gnu/11.3.0/include"

build:
	LIBCLANG_PATH=$(LIBCLANG_PATH) C_INCLUDE_PATH=$(C_INCLUDE_PATH) cargo build
.PHONY: build

run:
	@godot
.PHONY: run
