# sqlxinsert test

https://crates.io/crates/sqlxinsert

## run

run app against local postgres instance

```bash
docker run --name some-postgres  -p 5432:5432 -e POSTGRES_HOST_AUTH_METHOD=trust -d postgres
```
```bash
cargo run
```
