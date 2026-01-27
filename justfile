run:
    cargo run

dev:
    cargo watch -x run

build-debug:
    cargo build

build-release:
    cargo build --release

check:
    cargo check

clippy:
    cargo clippy

clippy-fix:
    cargo clippy --fix --allow-dirty --allow-staged

fmt:
    cargo fmt --all

check-fmt:
    cargo fmt --check

run-migrations:
    sqlx migrate run

create-migration name:
    sqlx migrate add {{name}}

revert-migration:
    sqlx migrate revert

run-seeders:
    grow run --all