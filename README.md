# Atomicflow Operator


## Dependency
- Docker: https://www.docker.com/
- Migration: https://crates.io/crates/refinery_cli

## Setup
1. Copy .env.sample as .env and fill in with the correct value

2. Startup containers

```shell
make docker-up
```

3. Migrate the database

```shell
make migrate
```

## Release build and run schedule script

To speed up proof generation, it is necessary to build at the production level. We will also describe the steps to run a script to create transactions at regular intervals.

1. Build

```shell
make build
```

2. Start the server

```shell
make start
```

3. Start the schedule script

```shell
make schedule
```
