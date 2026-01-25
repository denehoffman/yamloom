default:
    just --list

develop:
    CARGO_INCREMENTAL=true maturin develop --uv

test: develop
    cargo test
    uvx pytest

check: develop
    cargo clippy
    uvx ty check
    uvx ruff check
    uvx ruff format --check

fix: develop
    cargo clippy --fix --allow-dirty
    uvx ruff check --fix
    uvx ruff format

clean:
    cargo clean
    rm -r python/yamloom/_yamloom.abi3.so
