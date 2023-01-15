SYSDC_OPTS :=

# General
setup:
	make -C core setup
	make -C std/tool setup build

.PHONY: setup, build-server, run-server
