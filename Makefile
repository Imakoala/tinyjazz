all:
	cargo build --release
	cp target/release/tinyjazz tinyjazz

clean:
	cargo clean
	rm tinyjazz