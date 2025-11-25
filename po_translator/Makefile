.PHONY: api

dev:
	cargo watch -x run -i file.po
release:
	git pull;\
	cargo build --release 
default: dev
