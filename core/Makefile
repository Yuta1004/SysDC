IMAGE := sysdc/core:build

SRCS := $(shell find src/ -name "*.rs")

setup:
	docker build -t $(IMAGE) .

build: $(SRCS)
	docker run --rm -v $(shell pwd):/workdir $(IMAGE) \
		wasm-pack build --release --target web -- --features wasm
	sudo chown $(shell id -u):$(shell id -g) pkg pkg/*

clean:
	rm -rf pkg target	

.PHONY: setup clean
