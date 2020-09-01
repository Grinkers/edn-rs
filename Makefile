int:
	cargo test --test lib --no-fail-fast --features "json"

unit:
	cargo test --locked  --no-fail-fast --lib

ex:
	cargo test --examples
	cargo test --example json_to_edn --features "json"
	cargo run --example async --features "async"

test: unit int ex