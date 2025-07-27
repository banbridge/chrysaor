.PHONY: build
build:
	cargo build

.PHONY: run
run:
	RUST_LOG=debug RUST_BACKTRACE=1 cargo run

.PHONY: fmt
fmt:
	cargo fmt

.PHONY: install
install:
	cargo install sea-orm-cli

# .PHONY: gen-sea-orm
# gen-sea-orm:
# 	sea-orm-cli generate entity \
# 		--with-serde both \
# 		--model-extra-attributes 'serde(rename_all = "snake_case")' \
# 		--date-time-crate chrono \
# 	 	-o ./crates/infra/src/entity