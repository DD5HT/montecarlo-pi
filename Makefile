rust:
	cargo build --release

c-gcc:
	gcc -O3 -o target/pi-gcc src/bin/pi.c -lm

c-clang:
	clang -O3 -o target/pi-clang src/bin/pi.c -lm

bench-rust:
	hyperfine ./target/release/pi 

bench-rust-multi:
	hyperfine ./target/release/pi -m

bench-gcc:
	hyperfine ./target/pi-gcc

bench-clang:
	hyperfine ./target/pi-clang

clean:
	rm -rf target