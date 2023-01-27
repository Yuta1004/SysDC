IMAGE := sysdc/stdtool/view:build-wasm

SRCS := $(shell find src/) \
		$(shell find ../../../../core/src)

setup: Cargo.toml ../../../../core/Cargo.toml
	cp ../../../../core/Cargo.toml Cargo.core.toml
	docker build -t $(IMAGE) .
	rm Cargo.core.toml

build: Cargo.toml $(SRCS)
	docker run --rm -v $(shell pwd):/workdir -v $(shell pwd)/../../../../core:/workdir/core $(IMAGE) \
		wasm-pack build --release --target web
	sudo chown $(shell id -u):$(shell id -g) pkg pkg/*

clean:
	rm -rf pkg target

.PHONY: clean
