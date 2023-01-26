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
UNAME_S := $(shell uname -s)

.PHONY: package
package:
ifdef OS # windows
	mkdir -p target/package
	tar -a -cvf target/package/$(APP)-$(VERSION)-windows-x86_64-msvc.zip \
		-C $$PWD/target/x86_64-pc-windows-msvc/release $(APP).exe \
		-C $$PWD LICENSE README.md
else ifeq ($(UNAME_S),Darwin) # macOS
	mkdir -p target/package
	zip -j target/package/$(APP)-$(VERSION)-macos-x86_64.zip \
		target/x86_64-apple-darwin/release/$(APP) LICENSE README.md
else ifeq ($(UNAME_S),Linux) # linux
	sudo mkdir -p target/package
	sudo tar -z -cvf target/package/$(APP)-$(VERSION)-$(arch)-unknown-linux-$(libc).tar.gz \
		-C $$PWD/target/$(arch)-unknown-linux-$(libc)/release $(APP) \
		-C $$PWD LICENSE README.md
endif

version:
	@grep -rn --color \
		--exclude-dir ./target \
		--exclude-dir ./.git \
		--exclude Cargo.lock \
		--fixed-strings '$(VERSION)' .

.PHONY: publish
publish:
	cargo publish --manifest-path dateparser/Cargo.toml
