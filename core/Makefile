IMAGE := rust:1.63.0-sysdc-wasm

pkg:
	docker run --rm -v $(shell pwd):/workdir $(IMAGE) \
		wasm-pack build --release --target web --scope sysdc_core -- --features wasm
	sudo chown $(shell id -u):$(shell id -g) pkg pkg/*

build-image:
	docker build -t $(IMAGE) .

.PHONY: pkg build-image
