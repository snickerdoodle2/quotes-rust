set dotenv-load

watch:
    cargo watch -x run

migrate:
    sqlx migrate run
