IMAGE := node:18-alpine3.15

SRCS := $(shell find public/) \
		$(shell find src/)

setup: package.json
	docker run --rm -it -v $(shell pwd):/workdir -w /workdir $(IMAGE) /bin/sh -c "\
		npm install \
	"

build: package.json $(SRCS)
	make -C ../wasm build
	mkdir -p orig_modules/sysdc_tool_check
	cp -r ../wasm/pkg/* orig_modules/sysdc_tool_check
	docker run --rm -it -v $(shell pwd):/workdir -w /workdir $(IMAGE) /bin/sh -c "\
		npm run build \
	"
