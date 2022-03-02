
ROOT_DIR 	   := $(abspath $(lastword $(MAKEFILE_LIST)))
PROJECT_DIR	 := $(notdir $(patsubst %/,%,$(dir $(ROOT_DIR))))
PROJECT 		 := $(lastword $(PROJECT_DIR))
VERSION_FILE 	= VERSION
VERSION			 	= `cat $(VERSION_FILE)`

RUN_ARGS := $(wordlist 2,$(words $(MAKECMDGOALS)),$(MAKECMDGOALS))
$(eval $(RUN_ARGS):;@:)

default: watch
### Cargo

RUST_CHANNEL ?= stable
CARGO_FLAGS =
RUSTUP_INSTALLED = $(shell command -v rustup 2> /dev/null)

ifndef RUSTUP_INSTALLED
  CARGO = cargo
else
  ifdef CI
    CARGO = cargo
  else
    CARGO = rustup run $(RUST_CHANNEL) cargo
  endif
endif

.PHONY: help
help: ## Print all the available commands
	@echo "" \
	&& grep -E '^[a-zA-Z_-]+:.*?## .*$$' $(MAKEFILE_LIST) | \
	  awk 'BEGIN {FS = ":.*?## "}; {printf "\033[36m%-20s\033[0m %s\n", $$1, $$2}' \
	&& echo ""

check: ## Validate the project code
	@$(CARGO) check

build: ## Build the project in debug mode
	@$(CARGO) build $(CARGO_FLAGS)

release: CARGO_FLAGS += --release
release: build ## Build the project in release mode
	@cd ./target/release && \
	tar -czf motive-0.0.2-x86_64-apple-darwin.tar.gz motive


### Lints: https://rust-lang.github.io/rust-clippy/master/index.html

lint: fmt clippy ## Lint project files

fmt: ## Check the format of the source code
	@cargo fmt --all -- --check

clippy: ## Check the style of the source code and catch common errors
	@$(CARGO) clippy --all-targets --all-features -- -D warnings

.PHONY: test
test: ## Run Tests
	@$(CARGO) test --all-features $(TEST_FILTER)

dev: ## Run development
	@clear && $(CARGO) run --all-features -- example bleep bloop

docs: ## Build docs at target/doc
	@$(CARGO) doc

clean: ## Clean project
	@$(CARGO) clean && rm -rf target

.PHONY: watch
watch: ## Watch project and build on change
	@$(CARGO) watch -c -s "make dev"

### DEV ENV

install-watch:
	@$(CARGO) install cargo-watch

install-bump: ## make release (major|minor|patch)
	@$(CARGO) install cargo-bump

install-edit:
	@$(CARGO) install cargo-edit

.PHONY: setup
setup: install-edit install-watch install-bump ## Setup for development
	

