default:
    just --list

develop:
    CARGO_INCREMENTAL=true maturin develop --uv

test:
    cargo test
    uvx --with . pytest

check:
    cargo clippy
    uvx ty check
    uvx ruff check
    uvx ruff format --check

fix:
    cargo clippy --fix --allow-dirty
    uvx ruff check --fix
    uvx ruff format

clean:
    cargo clean
    rm -r python/yamloom/_yamloom.abi3.so

update-schema:
    curl -L -o schemas/github-workflow.json https://www.schemastore.org/github-workflow.json
