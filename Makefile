.PHONY : test run build clean
test:
	-@cargo test
run:
	-@cargo run
build:
	-@cargo build --release
clean:
	-@cargo clean


