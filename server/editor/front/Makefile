build:
	make -C ../../../core build
	mkdir -p ./orig_modules
	rm -rf ./orig_modules/sysdc_core
	cp -r ../../../core/pkg ./orig_modules/sysdc_core

clean:
	rm -rf node_modules orig_modules

.PHONY: build clean
