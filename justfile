root := `git rev-parse --show-toplevel`

_default:
    @just --list

_root:
    @echo {{root}}

_runtimes:
    @{{root}}/.scripts/runtimes.sh

act *args:
    @{{root}}/.scripts/act.sh {{args}}

build *args:
    @cargo build --workspace --all-targets --all-features {{args}}

run *args:
    @cargo run {{args}}

test *args:
    @cargo test --workspace --all-targets --all-features {{args}}

check *args:
    @cargo check {{args}}

fmt *args:
    @cargo fmt --all {{args}}

clippy *args:
    @cargo clippy --workspace --all-targets --all-features {{args}}

doc *args:
    @cargo doc --no-deps --document-private-items --all-features --workspace
