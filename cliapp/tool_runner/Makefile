IMAGE := node:18-alpine3.15

SRCS := $(shell find public/) \
		$(shell find src/)

setup: package.json
	make -C ../../std/tool setup
	docker run --rm -it -v $(shell pwd):/workdir -w /workdir $(IMAGE) /bin/sh -c "\
		npm install \
	"

build: package.json $(SRCS)
	docker run --rm -it -v $(shell pwd):/workdir -w /workdir $(IMAGE) /bin/sh -c "\
		npm run build \
	"
	make -C ../../std/tool build
	mkdir -p build/static/tools
	cp -r ../../std/tool/debug/build build/static/tools/debug
	cp -r ../../std/tool/json/build build/static/tools/json
	cp -r ../../std/tool/view/front/build build/static/tools/view
	cp -r ../../std/tool/eval/front/build build/static/tools/eval
	cp -r ../../std/tool/check/front/build build/static/tools/check

clean:
	rm -rf build node_modules

.PHONY: build clean
