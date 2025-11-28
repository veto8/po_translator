.PHONY: api

dev:
	cargo watch -x run -i *.po -i languages
release:
	git pull;\
	cargo build --release 
default: dev
