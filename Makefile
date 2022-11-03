all: bench

.PHONY: bench
bench:
	cargo criterion
	#CRITERION_DEBUG=1 cargo bench

.PHONY: setup
setup:
	cargo install cargo-criterion

clean:
	rm -rf target/criterion
