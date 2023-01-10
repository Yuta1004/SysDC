IMAGE := node:18-alpine3.15

dist:
	docker run --rm -it -v $(shell pwd):/workdir $(IMAGE) /bin/sh -c "\
		cd workdir && \
		npm install && \
		npm run build \
	"
	cp public/* dist

.PHONY: dist
