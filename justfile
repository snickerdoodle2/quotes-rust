set dotenv-load

watch:
    cargo watch -x run

run:
    cargo run

migrate:
    sqlx migrate run
