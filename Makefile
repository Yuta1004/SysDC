BUILD_OPTIONS :=

build:
	make -C tools/view
	cargo build $(BUILD_OPTIONS)
	cp ./target/debug/sysdc .

build-release:
	make -C tools/view
	cargo build --release $(BUILD_OPTIONS)
	cp ./target/release/sysdc .
