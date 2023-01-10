IMAGE := sysdc/core:build

setup:
	docker build -t $(IMAGE) .

build: src/
	docker run --rm -v $(shell pwd):/workdir $(IMAGE) \
		wasm-pack build --release --target web -- --features wasm
	sudo chown $(shell id -u):$(shell id -g) pkg pkg/*

.PHONY: setup
