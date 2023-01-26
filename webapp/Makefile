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
	@$(call docker-compose,up)

stop:
	@$(call docker-compose,stop)

down:
	@$(call docker-compose,down)

clean:
	make -C editor/front clean

define docker-compose
	SYSDC_BASE_URL=$(SYSDC_BASE_URL) \
	SYSDC_PORT=$(SYSDC_PORT) \
		docker-compose $1 $(SYSDC_OPTS)
endef

.PHONY: setup build run stop down clean
