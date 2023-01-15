SYSDC_OPTS :=

# General
setup:
	make -C core setup
	make -C std/tool setup

# Server-app
build-server:
	make -C std/tool build
	make -C server build

run-server:
	SYSDC_OPTS=$(SYSDC_OPTS) make -C server run

.PHONY: setup, build-server, run-server
