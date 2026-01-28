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
    cargo clippy --all-targets --all-features -- -D warnings

clippy-fix:
    cargo clippy --fix --allow-dirty --allow-staged

fmt:
    cargo fmt --all

check-fmt:
    cargo fmt --check

run-migrations:
    sqlx migrate run

create-migration name:
    sqlx migrate add {{ name }}

revert-migration:
    sqlx migrate revert

run-seeders:
    grow run --all

create-seeder name:
    grow new {{ name }}

module name:
    mkdir ./src/{{ name }}
    mkdir ./src/{{ name }}/dtos
    mkdir ./src/{{ name }}/entities
    mkdir ./src/{{ name }}/errors  
    touch ./src/{{ name }}/dtos/mod.rs
    touch ./src/{{ name }}/entities/mod.rs
    touch ./src/{{ name }}/errors/mod.rs  
    touch ./src/{{ name }}/mod.rs       
    touch ./src/{{ name }}/routes.rs   
    touch ./src/{{ name }}/service.rs
