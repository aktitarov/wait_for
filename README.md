[![Build Status](https://travis-ci.org/ktitarov/wait_for.svg?branch=master)](https://travis-ci.org/ktitarov/wait_for)

## Wait for another service to become available

`wait_for` is a rust based cli designed to synchronize services such as docker containers. It is highly inspired by [`eficode/wait-for`](https://github.com/eficode/wait-for), but rewritten to use with rust.

## Installation

To install the cli simple run the following command:

```
cargo install wait_for
```

## Usage

```
Waits for another service to become available

USAGE:
    wait_for [OPTIONS] --host <host> --port <port> [command]...

FLAGS:
        --help       Prints help information
    -V, --version    Prints version information

OPTIONS:
    -h, --host <host>          Sets host of service to be watched
    -p, --port <port>          Sets port of service to be watched
    -t, --timeout <timeout>    Sets timeout in secons, zero for no timeout

ARGS:
    <command>...    Command to execute after service is available
```

## Examples

To check if [google.com](https://google.com) is available:

```
wait_for -h www.google.com -p 80 -- echo "Google is up"
```

To wait for database container to become available:

```
version: '3'

service:
  database:
    image: postgres:latest
  app:
    build: .
    command: sh -c 'wait_for -h database -p 5432 -- ./target/release/app'
    depends_on:
      - database
```
 
## Note

Make sure netcat is installed in your Dockerfile before running the command.

```
RUN apt-get -q update && apt-get -qy install netcat
```

https://stackoverflow.com/questions/44663180/docker-why-does-wait-for-always-time-out