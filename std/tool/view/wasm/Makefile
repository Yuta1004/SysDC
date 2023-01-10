IMAGE := sysdc/stdtool/view:build-wasm

setup: ../../../../core/Cargo.toml Cargo.toml
	cp ../../../../core/Cargo.toml Cargo.core.toml
	docker build -t $(IMAGE) .
	rm Cargo.core.toml

build: ../../../../core/src/ src/ Cargo.toml
	docker run --rm -v $(shell pwd):/workdir -v $(shell pwd)/../../../../core:/workdir/core $(IMAGE) \
		wasm-pack build --release --target web
	sudo chown $(shell id -u):$(shell id -g) pkg pkg/*
