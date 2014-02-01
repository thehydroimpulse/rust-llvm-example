run: src/main.rs
	mkdir -p bin
	rustc src/bin.rs -o bin/rustllvm
	./bin/rustllvm
