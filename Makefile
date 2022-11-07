all: bench

BENCHMARK = par_solve_benchmark
BASELINE = master

.PHONY: bench
bench:
	cargo criterion
	#CRITERION_DEBUG=1 cargo bench

.PHONY: save
save:
	cargo bench --bench $(BENCHMARK) -- --save-baseline $(BASELINE)

.PHONY: baseline
baseline:
	cargo bench --bench $(BENCHMARK) -- --baseline $(BASELINE)

spbench.profile:
	cargo run --bin spbench

.PHONY: prof
prof: spbench.profile
	pprof -http=:8080 target/debug/spbench spbench.profile

.PHONY: setup
setup:
	cargo install cargo-criterion

clean:
	rm -rf target/criterion
