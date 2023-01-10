IMAGE := node:18-alpine3.15

setup:
	docker run --rm -it -v $(shell pwd):/workdir -w /workdir $(IMAGE) /bin/sh -c "\
		npm install \
	"

build:
	make -C ../wasm build
	mkdir -p orig_modules/sysdc_tool_view
	cp -r ../wasm/pkg/* orig_modules/sysdc_tool_view
	docker run --rm -it -v $(shell pwd):/workdir -w /workdir $(IMAGE) /bin/sh -c "\
		npm run build \
	"
	cp public/* build

.PHONY: setup, build
