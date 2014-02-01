run: src/main.rs
	mkdir -p bin
	rustc src/main.rs -o bin/rustllvm
	./bin/rustllvm
