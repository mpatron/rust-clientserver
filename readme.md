# test client/server

[docker rust](https://hub.docker.com/_/rust)
[docker alpine](https://hub.docker.com/_/alpine)

~~~bash
docker build -t rust-cs/rclient ./rclient
docker build -t rust-cs/rserver ./rserver
docker build --progress=plain --no-cache -t rust-cs/rserver .

docker run -it rust-cs/rclient bash
docker rm rserver || docker run --name rserver -a STDERR -p 3333:3333 rust-cs/rserver
docker run -it rust-cs/rserver sh
~~~

~~~bash
docker-compose build && docker-compose up
docker-compose run --rm rserver
docker-compose run --rm rclient # Too quick to go in
docker-compose down
~~~
