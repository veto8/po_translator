.PHONY: api

dev:
	cargo watch -x run -i de.po -i th.po
release:
	git pull;\
	cargo build --release 
default: dev
