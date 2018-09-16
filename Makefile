rust:
	cargo build --release

c-gcc:
	gcc -O3 -o target/pi-gcc src/bin/pi.c -lm

c-clang:
	clang -O3 -o target/pi-clang src/bin/pi.c -lm

bench-rust:
	time ./target/release/pi 

bench-gcc:
	time ./target/pi-gcc

bench-clang:
	time ./target/pi-clang

clean:
	rm -rf target