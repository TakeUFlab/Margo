alias c := check
alias b := build
alias r := run


# check all code in workspace
check:
    cargo check

# test all code in workspace
test:
    cargo test --all

# build all code in workspace
build:
    cargo build --features="serde cli" --release

# run src
run +arg:
    cargo run --bin margo --features="serde cli json" -- {{arg}}


# fmt all code in workspace
fmt:
    cargo fmt --all

# clippy all code in workspace
lint:
    cargo clippy --all --fix

# clean up all cache in workspace
clean:
    cargo clean