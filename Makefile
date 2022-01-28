DOCKER_IMAGE = venom_monkey:latest

.PHONY: all
all: agent client server

.PHONY: agent
agent: agent_linux agent_windows

.PHONY: agent_linux
agent_linux:
	cross build -p agent --release --target x86_64-unknown-linux-musl

.PHONY: agent_windows
agent_windows:
	cross build -p agent --release --target x86_64-pc-windows-gnu

.PHONY: client
server:
    cargo build -p client --release

.PHONY: server
server:
    cargo build -p server --release


.PHONY: fmt
fmt:
	cargo fmt

.PHONY: check
check:
	cargo check
