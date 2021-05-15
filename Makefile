build:
	cargo build --release
	cargo strip -o target.png -u e32d
update:
	
run:
	cargo build	
	./target/debug/jem-icon
help:
	@echo "usage: make $(prog) [debug=1]"cargo update
