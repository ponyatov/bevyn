.PHONY: all run
all: $(S)
	cargo build
run: $(S)
	cargo run -- $(S)
