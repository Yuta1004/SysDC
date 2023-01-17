USER := $(shell whoami)
IMAGE := klakegg/hugo:0.101.0-ext-alpine
FILEPATH := 

chapter:
	@$(call exec_on_hugo_container, new --kind chapter $(FILEPATH))

article:
	@$(call exec_on_hugo_container, new $(FILEPATH))

define exec_on_hugo_container
	docker run --rm -it -v $(shell pwd):/src --entrypoint /bin/bash $(IMAGE) -c "\
		hugo $1 && \
		addgroup $(USER) && \
		adduser -D -G $(USER) $(USER) && \
		chown -R $(USER):$(USER) /src/content \
	"
endef
