.SILENT:

BIN = ./target/release/agent
FLAGS = --mode

help:
	echo "Available commands:"
	echo "  make agent    - to run the agent, make sure that you created .end file"
	echo "  make interact - to run the cli to interact with agent"
	echo "  make build    - build the binary"
	echo "  make clean    - clean all binaries(remove artifacts from the target directory)"
	echo "  make help     - show this message"

agent: build
	$(BIN) $(FLAGS) agent

interact: build
	$(BIN) $(FLAGS) interaction

build:
	cargo build --release

clean:
	cargo clean
