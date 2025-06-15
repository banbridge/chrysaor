.PHONY: build
build:
	cargo build

.PHONY: run
run:
	cargo run

.PHONY: fmt
fmt:
	cargo fmt

.PHONY: install
install:
	cargo install sea-orm-cli

.PHONY: gen-sea-orm
gen-sea-orm:
	sea-orm-cli generate entity \
		--with-serde both \
		--model-extra-attributes 'serde(rename_all = "snake_case")' \
		--date-time-crate chrono \
	 	-o ./crates/infra/src/entity