Chinese-English vocabulary
======
Simple implementation of CRUD in Rust for a vocabulary app

============
Home Page:
---------

<img width="773" alt="Screenshot 2022-09-07 at 16 23 39" src="https://user-images.githubusercontent.com/81642088/188832479-6d958225-bb69-4ae3-b59b-f1e4b0a2253f.png">

Create Page:
---------
<img width="765" alt="Screenshot 2022-09-07 at 16 23 51" src="https://user-images.githubusercontent.com/81642088/188832524-0bc6932d-335e-4a34-a014-8fc971b57a6c.png">

Detail Page:
---------
<img width="613" alt="Screenshot 2022-09-07 at 16 36 00" src="https://user-images.githubusercontent.com/81642088/188832563-e0c2d059-a37e-458b-b4c9-e407ebd1ff9c.png">


============


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
