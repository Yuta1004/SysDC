# General
setup:
	make -C core setup
	make -C std/tool setup build

clean:
	make -C core clean
	make -C server clean
	make -C std/tool clean

.PHONY: setup clean
