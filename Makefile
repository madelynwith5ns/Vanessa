.PHONY: all

all: rust

rust:
	@echo Building Vanessa-RS \(crate: vanessa\)
	cd vanessa-rs && cargo build --release
