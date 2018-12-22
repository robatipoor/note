SHELL := /bin/bash
COMPILER = rustc
COMPILER_FLAGS = -O
RUSTDOC = rustdoc
UPX := $(shell command -v upx 2> /dev/null)

.PHONY: all
all:
	cargo build --release 
	strip target/release/note-rs
ifdef UPX
		upx target/release/note-rs
endif
	cargo install --path . 

.PHONY: build
build:
	cargo build --release 
	strip target/release/note-rs
	upx target/release/note-rs 

.PHONY: run
run:
	cargo run

.PHONY: install
install:
	cargo install --path .

.PHONY: clean
clean:
	cargo clean