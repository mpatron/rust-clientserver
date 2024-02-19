# test client/server

[docker rust](https://hub.docker.com/_/rust)
[docker alpine](https://hub.docker.com/_/alpine)

~~~bash
~/rust-cs/rclient$ docker build -t rust-cs/rclient .
~/rust-cs/rclient$ docker build -t rust-cs/rserver .
~/rust-cs/rserver$ docker build --progress=plain --no-cache -t rust-cs/rserver .

docker run -it rust-cs/rclient bash
docker rm rserver || docker run --name rserver -a STDERR -p 3333:3333 rust-cs/rserver
docker run -it rust-cs/rserver sh
~~~
