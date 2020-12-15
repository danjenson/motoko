build-MotokoGraphQL:
	cargo build --release --target x86_64-unknown-linux-musl
	cp ./target/x86_64-unknown-linux-musl/release/graphql $(ARTIFACTS_DIR)/bootstrap

build-MotokoGarbageCollect:
	cargo build --release --target x86_64-unknown-linux-musl
	cp ./target/x86_64-unknown-linux-musl/release/garbage-collect $(ARTIFACTS_DIR)/bootstrap
