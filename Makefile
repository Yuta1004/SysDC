BASE_URL := http://localhost:50000
PORT := 50000
OPTS :=

build:
	make -C core build-image
	make -C server build

run:
	SYSDC_BASE_URL=$(BASE_URL) \
	SYSDC_PORT=$(PORT) \
	SYSDC_OPTS=$(OPTS) \
		make -C server run

.PHONY: build, run
