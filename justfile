root := `git rev-parse --show-toplevel`

_default:
    @just --list

_root:
    @echo

_runtimes:
    @/.scripts/runtimes.sh

act *args:
    @/.scripts/act.sh


test:
    @cargo test --workspace --all-targets --all-features -- --nocapture

test-release:
    @cargo test --release --workspace --all-targets --all-features -- --nocapture

check:
    @cargo check

check-release:
    @cargo check --release

fmt:
    @cargo fmt --all

fmt-check:
    @cargo fmt --all -- --check

clippy:
    @cargo clippy --workspace --all-targets --all-features
