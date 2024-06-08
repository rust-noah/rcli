# Run rust examples
.PHONY: clap subcommand mode validated_values
 
clap:
	cargo run --example clap -- $(ARGS) # USAGE: make clap ARGS="--help"

subcommand:
	cargo run --example subcommand -- $(ARGS)

mode:
	cargo run --example mode -- $(ARGS)

validated_values:
	cargo run --example validated_values -- $(ARGS)

run:
	cargo run -- $(ARGS)