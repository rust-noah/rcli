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

text-cli:
	cargo run --example text_cli -- $(ARGS)
async:
	cargo run --example async
async2:
	cargo run --example async2
axum:
	cargo run --example axum
thread: 
	cargo run --example thread

run:
	@cargo run -- $(ARGS)
# ******** csv ********
# make run ARGS="csv --input "
# make run ARGS="csv --input a.csv"
# make run ARGS="csv --input ./asserts/juventus.csv"
# make run ARGS="csv --input './asserts/juventus.csv'"
# make run ARGS="csv --input ./assets/juventus.csv --output ./assets/juventus.csv"
# make run ARGS="csv --input ./assets/juventus.csv --output ./assets/juventus.json"
# make run ARGS="csv --input ./assets/juventus.csv --format json"
# make run ARGS="csv -h"
# make run ARGS="csv --input ./assets/juventus.csv --format toml"
# make run ARGS="csv --input ./assets/juventus.csv --format yaml --output ./assets/juventus"
# make run ARGS="csv --input ./assets/juventus.csv --format json --output ./assets/juventus"
# make run ARGS="csv --input ./assets/juventus.csv --format yaml"

# ******** genpass ********
# make run ARGS="genpass -h"
# make run ARGS="genpass"
# make run ARGS="genpass -l 20"
# make run ARGS="genpass -l 30"
# make run ARGS="genpass -l 100"
# make run ARGS="genpass -l 3"
# make run ARGS="genpass -l 4^J"
# make run ARGS="genpass -l 116"
# make run ARGS="genpass -l 16"
# make run ARGS="genpass --upper --lower --number 16 --symbool"
# make run ARGS="genpass --upper --lower --symbool --number 16"
# make run ARGS="genpass --upper --lower --symbol --number 16"
# make run ARGS="genpass --upper --lower --symbol -n16"
# make run ARGS="genpass --upper --lower --symbol"
# make run ARGS="genpass --upper --lower --symbol -n 16"
# make run ARGS="genpass --length 100 --upper --lower --symbol --number"
# make run ARGS="genpass --length 10 --upper --lower --symbol --number"
# make run ARGS="genpass --length 16 --upper --lower --symbol --number"
# make run ARGS="genpass -l 32"

# ******** base64 ********
# make run ARGS="base64 encode"
# make run ARGS="base64"
# make run ARGS="base64 decode -i 
# make run ARGS="base64 decode -i Cargo.b64"
# make run ARGS="base64 encode -i Cargo.toml"
# make run ARGS="base64 encode -i Cargo.toml" > ./fixtures/b64.txt
# make run ARGS="base64 encode"
# make run ARGS="base64 decode"
# make run ARGS="base64 decode -i ./fixtures/b64.txt"
# make run ARGS="base64 encode -i Cargo.toml --format urlsafe" > ./fixtures/b64.txt   
# make run ARGS="base64 decode -i ./fixtures/b64.txt --format urlsafe"

# ******** text ********
# make run ARGS="text sign --format ed25519 --key fixtures/blake3.txt"
# make run ARGS="text generate --format ed25519 -o ./fixtures/ed25519"
# make run ARGS="text generate --format ed25519 -o ./fixtures"
# make run ARGS="text generate --format blake3 -o ./fixtures"
# make run ARGS="text verify -k ./fixtures/blake3.txt"
# make run ARGS="text verify -k ./fixtures/blake3.txt --sig"
# make run ARGS="text verify -h"
# make run ARGS="text verify -k ./fixtures/blake3.txt --sig 'eG6j-ghHZvMgTj0fQRrHrQ17v"
# make run ARGS="text verify -k ./fixtures/blake.txt --sig eG6j-ghHZvMgTj0fQRrHrQ17vdo"
# make run ARGS="text sign --format blake3 --key fixtures/blake3.txt"
# make run ARGS="text verify -k ./fixtures/blake3.txt --sig'eG6j-ghHZvMgTj0fQRrHrQ17vd"
# make run ARGS="text verify -k ./fixtures/blake3.txt --sig 'eG6j-ghHZvMgTj0fQRrHrQ17v"
# make run ARGS="text sign -h"
# make run ARGS="text verify --format ed25519 -k ./fixtures/ed25519.pk"
# make run ARGS="text verify --format ed25519 -k ./fixtures/ed25519.pk --sig yY0cc2vo"
# make run ARGS="text sign --format ed25519 -k ./fixtures/ed25519.pk"
# make run ARGS="text verify --format ed25519 -k ./fixtures/ed25519.sk --sig B6SBry_iU"
# make run ARGS="text sign --format ed25519 -k ./fixtures/ed25519.sk"
# make run ARGS="text verify --format ed25519 -k ./fixtures/ed25519.pk --sig 'yY0cc2vo"

# ******** http ********
# make run ARGS="http serve"
