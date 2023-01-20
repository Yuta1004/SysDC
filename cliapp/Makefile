BUILD_OPTIONS :=

build:
	make -C tool_runner
	cargo build $(BUILD_OPTIONS)
	cp ./target/debug/sysdc_cli .

build-release:
	make -C tool_runner
	cargo build --release $(BUILD_OPTIONS)
	cp ./target/release/sysdc_cli .
