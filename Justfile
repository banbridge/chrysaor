build:
    cargo build

clean:
    cargo clean

clean_and_build: clean build

bacon_run:
    bacon run-long

generate:
    buf generate .

init:
    cargo install sea-orm-cli@^2.0.0-rc
    cargo install protoc-gen-prost --version 0.5.0 --root .buf-deps/plugins/ --force
    cargo install protoc-gen-prost-serde --version 0.4.0 --root .buf-deps/plugins/ --force
    cargo install protoc-gen-tonic --version 0.5.0 --root .buf-deps/plugins/ --force
    cargo install protoc-gen-prost-crate --version 0.5.0 --root .buf-deps/plugins/ --force
    cargo install protoc-gen-prost-validate --version 0.0.1 --root .buf-deps/plugins/ --force

info_dep package:
    cargo info {{ package }} --registry crates-io

gen_entity:
    # Generate entity files of database `bakery` to `./src/entity`
    sea-orm-cli generate entity \
        --output-dir ./crates/infra/src/entity \
        --entity-format dense
