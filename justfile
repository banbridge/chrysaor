build:
    cargo build

clean:
    cargo clean

build_and_clean: build clean


bacon_run:
    bacon run-long