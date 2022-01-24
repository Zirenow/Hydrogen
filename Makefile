
build:
	cargo build --release

run:
	cargo run

clean:
	cargo clean 

install:
	cargo install --path $(CURDIR)
