.PHONY: help
help:
	@grep -E '^[a-zA-Z0-9_-]+:.*?## .*$$' $(MAKEFILE_LIST) | sort | awk 'BEGIN {FS = ":.*?## "}; {printf "\033[36m%-30s\033[0m %s\n", $$1, $$2}'

.PHONY: build
build: ## Build release with cargo for the current OS
	cargo build --release

.PHONY: lint
lint: ## Run clippy linter
	cargo clippy --workspace --tests --all-features -- -D warnings

.PHONY: test
test: ## Run unit tests
	RUST_BACKTRACE=1 cargo test

.PHONY: cover
cover: ## Generate test coverage report
	docker run \
        --security-opt seccomp=unconfined \
        -v ${PWD}:/volume \
		-e "RUST_BACKTRACE=1" \
		xd009642/tarpaulin \
        cargo tarpaulin --color auto --out Html --output-dir ./target
	open target/tarpaulin-report.html

.PHONY: bench
bench: ## Generate benchmark report
	cargo bench --bench parse -- --verbose
	open target/criterion/report/index.html

APP = belt
VERSION := $(shell cargo metadata -q | jq -r '.packages[] | select(.name == "$(APP)") | .version')
UNAME_S := $(shell uname -s)
NEXT_VERSION := $(shell echo "$(VERSION)" | awk -F. -v OFS=. '{$$NF += 1 ; print}')

.PHONY: package
package: ## Make release package based on the current OS
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

.PHONY: show-version-files
show-version-files: ## Find all files with the current version
	@grep -rn --color \
		--exclude-dir={target,.git} \
		--exclude Cargo.lock \
		--fixed-strings '"$(VERSION)"' .

.PHONY: bump-version
bump-version: ## Bump version in files that contain the current version
	@echo "Bumping version $(VERSION) -> $(NEXT_VERSION)..."
	@for file in $(shell grep -rl --exclude-dir={target,.git} --exclude Cargo.lock --fixed-strings '"$(VERSION)"' .); do \
		echo "In file $$file"; \
		sed -i '' -e 's/$(VERSION)/$(NEXT_VERSION)/g' $$file; \
	done
	@echo
	@echo "Bumped version in the following files:"
	@make show-version-files

.PHONY: release
release: ## Make a new tag based on the version from Cargo.toml and push to GitHub
	@if [[ "$(shell git tag -l)" == *"v$(VERSION)"* ]]; then \
		echo "Tag v$(VERSION) already exists"; \
	else \
		echo "Tagging v$(VERSION) and pushing to GitHub..."; \
		git tag -a v$(VERSION) -m "Release v$(VERSION)"; \
		git push origin v$(VERSION); \
	fi

.PHONY: publish
publish: ## Publish to crates.io
	cargo publish --manifest-path dateparser/Cargo.toml --token $(token)
