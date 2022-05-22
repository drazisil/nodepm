all: docs

docs:
	cargo doc --no-deps
	rm -rf docs && mkdir -p docs
	cp -r target/doc/ docs/

.PHONY: docs