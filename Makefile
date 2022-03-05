
build:
	cargo build --release

run:
	cargo run

clean:
	cargo clean 

install_windows:
	cargo install --path $(CURDIR)

install_linux:
	cargo install --path $(CURDIR) --root /usr/local --force