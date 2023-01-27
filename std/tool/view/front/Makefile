IMAGE := node:18-alpine3.15

SRCS := $(shell find public/) \
		$(shell find src/)

setup: package.json
	docker run --rm -v $(shell pwd):/workdir -w /workdir $(IMAGE) /bin/sh -c "\
		npm install \
	"

build: package.json $(SRCS)
	make -C ../wasm build
	mkdir -p orig_modules/sysdc_tool_view
	cp -r ../wasm/pkg/* orig_modules/sysdc_tool_view
	docker run --rm -v $(shell pwd):/workdir -w /workdir $(IMAGE) /bin/sh -c "\
		npm run build \
	"

clean:
	rm -rf build node_modules orig_modules

.PHONY: clean
