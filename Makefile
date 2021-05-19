.PHONY: build
build:
	cargo build --release

.PHONY: lint
lint:
	cargo clippy --workspace --tests --all-features -- -D warnings

.PHONY: test
test:
	RUST_BACKTRACE=1 cargo test

.PHONY: cover
cover:
	docker run \
        --security-opt seccomp=unconfined \
        -v ${PWD}:/volume \
		-e "RUST_BACKTRACE=1" \
		xd009642/tarpaulin \
        cargo tarpaulin --color auto --out Html --output-dir ./target
	open target/tarpaulin-report.html

.PHONY: cross
cross: build
	docker build -t $(APP)/cross -f cross.Dockerfile .
	docker run -it --rm \
		-v /var/run/docker.sock:/var/run/docker.sock \
		-v $$PWD:/src/$(APP) \
		-w /src/$(APP) \
		$(APP)/cross \
		/bin/bash -c ' \
			cross build --target x86_64-unknown-linux-gnu --release && \
			cross build --target x86_64-unknown-linux-musl --release && \
			cross build --target armv7-unknown-linux-gnueabihf --release \
		'

.PHONY: bench
bench:
	cargo bench --bench parse -- --verbose
	open target/criterion/report/index.html

APP = belt
VERSION := $(shell cargo metadata -q | jq -r '.packages[] | select(.name == "$(APP)") | .version')
MACOS_X86_64 := target/package/$(APP)-$(VERSION)-x86_64-apple-darwin.tar.gz
LINUX_DYN_X86_64 := target/package/$(APP)-$(VERSION)-x86_64-unknown-linux-gnu.tar.gz
LINUX_STAT_X86_64 := target/package/$(APP)-$(VERSION)-x86_64-unknown-linux-musl.tar.gz
LINUX_ARMV7 := target/package/$(APP)-$(VERSION)-armv7-unknown-linux-gnueabihf.tar.gz

.PHONY: release
release: cross
	@echo "[release] Cleaning up before packaging..."
	mkdir -p target/package
	rm -f $(MACOS_X86_64) $(LINUX_X86_64) $(LINUX_ARMV7)
	@echo "[release] Creating package for MacOS x86_64..."
	tar -cvzf $(MACOS_X86_64) \
		-C $$PWD/target/release $(APP)
	@echo "[release] Creating package for Linux (dynamic) x86_64..."
	tar -cvzf $(LINUX_DYN_X86_64) \
		-C $$PWD/target/x86_64-unknown-linux-gnu/release $(APP)
	@echo "[release] Creating package for Linux (static) x86_64..."
	tar -cvzf $(LINUX_STAT_X86_64) \
		-C $$PWD/target/x86_64-unknown-linux-musl/release $(APP)
	@echo "[release] Creating package for Linux armv7..."
	tar -cvzf $(LINUX_ARMV7) \
		-C $$PWD/target/armv7-unknown-linux-gnueabihf/release $(APP)
	@make checksum

.PHONY: checksum
checksum:
	shasum -a 256 target/package/$(APP)-$(VERSION)-*.tar.gz > target/package/$(APP)-$(VERSION)-checksums.txt

.PHONY: publish
publish:
	cargo publish --manifest-path dateparser/Cargo.toml
