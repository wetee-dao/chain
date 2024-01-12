
### Run in Docker

First, install [Docker](https://docs.docker.com/get-docker/) and
[Docker Compose](https://docs.docker.com/compose/install/).  

Then run the following command to start a single node development chain.  

A few useful ones are as follow:  

```bash
docker build -f ./pos_build.Dockerfile -t wetee/wetee-node:dev .

# Run node without re-compiling
docker run -p 9944:9944 wetee/wetee-node:dev
```