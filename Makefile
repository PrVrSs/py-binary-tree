SHELL := /usr/bin/env bash

.PHONY: build
build: dev-packages
	poetry run maturin build

.PHONY: develop
develop: dev-packages
	poetry run maturin develop --release

.PHONY: dev-packages
dev-packages:
	poetry install

.PHONY: test
test:
	poetry run pytest -v \
					  -vv \
					  tests

.PHONY: clean
clean:
	cargo clean

.PHONY: lint
lint:
	cargo clippy
