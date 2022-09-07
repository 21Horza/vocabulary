Chinese-English vocabulary
======
Simple implementation of CRUD in Rust for a vocabulary app

<img width="773" alt="Screenshot 2022-09-07 at 16 23 39" src="https://user-images.githubusercontent.com/81642088/188832479-6d958225-bb69-4ae3-b59b-f1e4b0a2253f.png">

---------------------

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
