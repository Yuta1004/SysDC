build:
	make -C core build-image
	make -C editor/front sysdc_core_pkg
