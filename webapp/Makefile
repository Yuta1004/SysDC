SYSDC_BASE_URL := http://localhost:50000
SYSDC_PORT := 50000

-include run.conf

setup:
	make -C ../core setup

build:
	make -C editor/front build
	echo "SYSDC_BASE_URL = \"http://localhost:50000\"" > run.conf
	echo "SYSDC_PORT = 50000" >> run.conf

run:
	SYSDC_BASE_URL=$(SYSDC_BASE_URL) \
	SYSDC_PORT=$(SYSDC_PORT) \
		docker-compose up $(SYSDC_OPTS)

clean:
	make -C editor/front clean

.PHONY: setup build run clean
