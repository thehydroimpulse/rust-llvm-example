run: src/bin.rs
	mkdir -p target
	rustc src/bin.rs --out-dir target
	./target/bin
