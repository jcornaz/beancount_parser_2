export RUSTFLAGS := "-D warnings"
export RUSTDOCFLAGS := "-D warnings"

@_choose:
	just --choose --unsorted

# Prepare the repository for development (requires rustup to be installed)
setup: && install-git-hooks
	rustup install stable
	rustup update stable
	rustup override set stable
	cargo install cargo-hack cargo-watch

# Perform all verifications (compile, test, lint, etc.)
verify: doc lint test

# Watch changes, and run `just verify` when source changes
watch:
	cargo watch -s 'just verify'

# Install a git hook to run tests before every commits
install-git-hooks:
	echo "#!/usr/bin/env sh" > .git/hooks/pre-commit
	echo "just verify" >> .git/hooks/pre-commit
	chmod +x .git/hooks/pre-commit

# Run all tests
test:
	cargo hack test --feature-powerset

# Static code analysis
lint:
	cargo fmt -- --check
	cargo clippy --all-features --all-targets

# Build the documentation
doc:
	cargo doc --all-features --no-deps

# Clean up compilation output
clean:
	rm -rf target
	rm -f Cargo.lock
	rm -rf node_modules

# run the release process in dry run mode (requires npm and a `GITHUB_TOKEN`)
release-dry-run: (release "--dry-run")

# Run the release process (requires `npm`, a `GITHUB_TOKEN` and a `CARGO_REGISTRY_TOKEN`)
release *args:
	npm install --no-save conventional-changelog-conventionalcommits @semantic-release/exec
	npx semantic-release {{args}}