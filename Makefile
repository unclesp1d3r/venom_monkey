DOCKER_IMAGE = venom_monkey:latest

.PHONY: all
all: build docker

.PHONY: build
build: agent client server

.PHONY: build_all
build_all: agent_all client server

.PHONY: agent_all
agent_all: agent_macos agent_linux agent_windows

.PHONY: agent
agent:
	cargo build -p agent --release
	upx -9 target/release/agent
	mv target/release/agent target/agent.native

.PHONY: agent_linux
agent_linux:
	cross build -p agent --release --target x86_64-unknown-linux-musl
	upx -9 target/x86_64-unknown-linux-musl/release/agent
	mv target/x86_64-unknown-linux-musl/release/agent target/agent.linux_x86_64

.PHONY: agent_windows
agent_windows:
	cross build -p agent --release --target x86_64-pc-windows-gnu
	upx -9 target/x86_64-pc-windows-gnu/release/agent
	mv target/x86_64-pc-windows-gnu/release/agent target/agent.windows_x86_64

.PHONY: agent_macos
agent_macos:
	cross build -p agent --release --target aarch64-apple-darwin

.PHONY: client
client:
	cargo build -p client --release

.PHONY: server
server:
	cargo build -p server --release

.PHONY: docker
docker:
	docker build -t $(DOCKER_IMAGE) .

.PHONY: dev
dev:
	cargo watch -x 'run -p server'

.PHONY: fmt
fmt:
	cargo fmt

.PHONY: clean
clean:
	cargo clean
