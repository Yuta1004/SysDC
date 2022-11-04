sysdc_core_pkg:
	make -C ../../core pkg
	rm -rf ./node_modules/@sysdc_core
	mv ../../core/pkg $(shell pwd)/node_modules/@sysdc_core

.PHONY: sysdc_core_pkg
