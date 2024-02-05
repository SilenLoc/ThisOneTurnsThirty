
set dotenv-load


alias r := run

bt := '0'

log := "warn"

export JUST_LOG := log
export ROCKET_PORT := "3721"

@_list:
	just --list --unsorted

frontend *args:
    cd frontend/tbtt && just {{args}}

hurl_opts := "--variables-file hurl.env.test --test"



# Perform all verifications (compile, test, lint, etc.)
verify: test shutdown run-release api-test lint

# Run the service locally (from sources)
run:
	cargo-shuttle run --port 3721


# Run the tests
test:
	cargo hack test --feature-powerset --locked
	cargo deny check licenses

# Run the static code analysis
lint:
	cargo fmt --all -- --check
	cargo hack clippy --feature-powerset --all-targets --workspace --locked
	cargo deny check

wait-for-api:
	hurl api/health.hurl --retry 60 {{hurl_opts}}


# run acceptance tests against the running test stack
api-test *args: wait-for-api
    hurl api/*.hurl {{hurl_opts}} {{args}}

# Install cargo dev-tools used by the `verify` recipe (requires rustup to be already installed)
install-dev-tools:
	rustup install stable
	rustup override set stable
	cargo install cargo-hack cargo-watch cargo-deny hurl


fmt:
  cargo fmt


shutdown:
    lsof -t -i:{{ROCKET_PORT}} | xargs -r kill

run-release: shutdown
    just run &