build:
	make -C core build-image
	make -C server build

conf:
	echo "SYSDC_BASE_URL = \"http://localhost:50000\"" > run.conf
	echo "SYSDC_PORT = 50000" >> run.conf
	echo "SYSDC_OPTS =" >> run.conf

run:
	make -C server run

.PHONY: build, run
