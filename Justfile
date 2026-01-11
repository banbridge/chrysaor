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

build_image_base version:
    cd docker && ls \
    && docker login --username=banbridge registry.cn-beijing.aliyuncs.com \
    && docker buildx build --progress=plain --platform linux/amd64 -t registry.cn-beijing.aliyuncs.com/banbridge/build_base:{{ version }}-amd64 --push . \
    && docker buildx build --progress=plain --platform linux/arm64 -t registry.cn-beijing.aliyuncs.com/banbridge/build_base:{{ version }}-arm64 --push . \
    && docker manifest create registry.cn-beijing.aliyuncs.com/banbridge/build_base:{{ version }} \
    registry.cn-beijing.aliyuncs.com/banbridge/build_base:{{ version }}-amd64 \
    registry.cn-beijing.aliyuncs.com/banbridge/build_base:{{ version }}-arm64  \
    && docker manifest push registry.cn-beijing.aliyuncs.com/banbridge/build_base:{{ version }}
