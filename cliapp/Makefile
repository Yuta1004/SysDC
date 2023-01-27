BUILD_OPTIONS :=

setup:
	make -C tool_runner setup

build:
	make -C tool_runner build
	cargo build --release $(BUILD_OPTIONS)
	cp ./target/release/sysdc_cli .

clean:
	make -C tool_runner clean

.PHONY: setup build clean
