TEE = 2>&1 | tee make.out
TEE_A = 2>&1 | tee -a make.out

all:	build

.PHONY: build
build:
	cargo build $(TEE)

.PHONY: run
run:
	cargo run $(TEE)

.PHONY: clean
clean:
	cargo clean
