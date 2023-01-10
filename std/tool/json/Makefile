IMAGE := node:18-alpine3.15

setup:
	docker run --rm -it -v $(shell pwd):/workdir -w /workdir $(IMAGE) /bin/sh -c "\
		npm install \
	"

build:
	docker run --rm -it -v $(shell pwd):/workdir -w /workdir $(IMAGE) /bin/sh -c "\
		npm run build \
	"

.PHONY: setup, build
