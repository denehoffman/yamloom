default:
    just --list

develop:
    CARGO_INCREMENTAL=true maturin develop --uv

test:
    cargo test
    uvx pytest

check:
    cargo clippy
    uvx ty check
    uvx ruff check
    uvx ruff format --check

fix:
    cargo clippy --fix
    uvx ruff check --fix
    uvx ruff format
