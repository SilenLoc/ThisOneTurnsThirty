
set dotenv-load


alias r := run

bt := '0'

log := "warn"

export JUST_LOG := log
export ROCKET_PORT := "3721"
# export LIBSQL_URL="libsql://eminent-asgardian-silenloc.turso.io"



@_list:
	just --list --unsorted

frontend *args:
    cd frontend/tbtt && just {{args}}

hurl_opts := "--variables-file hurl.env.test --test"

# Perform all verifications (compile, test, lint, etc.)
@verify: test run-release api-test lint
	just shutdown
	echo --- all good ---

commit message:
	git add .
	git commit -m {{message}}
	git push

# Run the service locally (from sources)
@run:
	cargo-shuttle run --port 3721

# Run the tests
@test:
	cargo hack test --feature-powerset --locked
	echo ---test ok---
	echo
	cargo deny check licenses
	echo ---licenses ok---
	echo

# Run the static code analysis
@lint:
	cargo fmt --all -- --check
	echo ---format ok---
	echo
	cargo hack clippy --feature-powerset --all-targets --workspace --locked
	echo ---clippy okay---
	echo
	cargo deny check
	echo ---deny okay---
	echo

@wait-for-api:
	hurl api_tests/healthz.hurl --retry 60 {{hurl_opts}}


# run acceptance tests against the running test stack
@api-test *args: wait-for-api
    hurl api_tests/*.hurl {{hurl_opts}} {{args}}
    echo ---api ok---
    echo

# Install cargo dev-tools used by the `verify` recipe (requires rustup to be already installed)
install-dev-tools:
	rustup install stable
	rustup override set stable
	cargo install cargo-hack cargo-watch cargo-deny hurl


deploy:
	cargo-shuttle deploy

fmt:
  cargo fmt


@shutdown:
    lsof -t -i:{{ROCKET_PORT}} | xargs -r kill

@run-release: shutdown
    just run &