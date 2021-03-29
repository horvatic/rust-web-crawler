.PHONY: build
.NOTPARALLEL:
build: clean
	cargo build

.PHONY: clean
clean:
	rm -rf target/ Cargo.lock
