setup:
	make -C core setup
	make -C std/tool setup

build:
	make -C std/tool build
	make -C server build

run:
	SYSDC_OPTS=$(SYSDC_OPTS) make -C server run

conf:
	echo "SYSDC_BASE_URL = \"http://localhost:50000\"" > run.conf
	echo "SYSDC_PORT = 50000" >> run.conf

.PHONY: build, run
