TEE = 2>&1 | tee make.out
TEE_A = 2>&1 | tee -a make.out

all:	build

.PHONY: build
build:
	clear
	cargo build $(TEE)

.PHONY: run
run:
	clear
	cargo run -- test/test1.xtce $(TEE)

.PHONY: test
test:
	@set -eu; set -x; \
		c_file="$$(mktemp --suffix .c)"; \
		o_file="$$(mktemp --suffix .o)"; \
		cargo run -- test/test1.xtce >$$c_file; \
		cc -c -o $$o_file $$c_file; \
		rm -f $$c_file $$o_file

.PHONY: clean
clean:
	cargo clean
