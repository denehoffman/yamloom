default:
    just --list

develop:
    CARGO_INCREMENTAL=true maturin develop --uv

check:
    ty check
    ruff check
    ruff format

fix:
    ruff check --fix
