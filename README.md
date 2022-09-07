Chinese-English vocabulary
======
Simple implementation of CRUD in Rust for a vocabulary app


How to use
======

## DB

```sh
# Start the DB
docker run --rm -p 7878:5432 -e "POSTGRES_PASSWORD=postgres" --name pg postgres:14

# optional psql (other terminal)
docker exec -it -u postgres pg psql
```

## server

```sh
# Start the server
cd backend && cargo run
```


## client

```sh
# Start the client
cd frontend && trunk serve
```