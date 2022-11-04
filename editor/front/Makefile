sysdc_core_pkg:
	make -C ../../core pkg
	mkdir -p ./node_modules
	rm -rf ./node_modules/@sysdc_core
	mv ../../core/pkg ./node_modules/@sysdc_core

.PHONY: sysdc_core_pkg
