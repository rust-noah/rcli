# Run rust examples
.PHONY: clap

clap:
	cargo run --example clap -- $(ARGS) # USAGE: make clap ARGS="--help"