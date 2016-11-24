# Drop-in Makefile from https://gist.github.com/mzabaluev/502307bb41ba6268594f
CARGO = cargo

CARGO_OPTS =

all:
	$(MAKE) build
	$(MAKE) doc

build:
	$(CARGO) $(CARGO_OPTS) build --verbose

clean:
	$(CARGO) $(CARGO_OPTS) clean

check:
	$(MAKE) build --verbose
	$(MAKE) test --verbose

test:
	$(CARGO) $(CARGO_OPTS) test  --verbose

bench:
	$(CARGO) $(CARGO_OPTS) bench

doc:
	$(CARGO) $(CARGO_OPTS) doc

release:
	$(CARGO) $(CARGO_OPTS) build --release

.PHONY: all build clean check test bench doc release