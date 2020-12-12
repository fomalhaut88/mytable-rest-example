A plain REST API that includes [rocket](https://rocket.rs/) as a REST framework
and [mytable](https://github.com/fomalhaut88/mytable) as a database together.

## Start web service

1. Build:

```
cargo build --release
```

2. Start:

```
ROCKET_ENV=prod ./target/release/mytable-rest-example
```

## Load test

As far as the service creates the file for the table **person.tbl**, it is
necessary to clean it before each load test. So use this command to start:

```
cargo build --release && rm -f person.tbl && ROCKET_ENV=prod ./target/release/mytable-rest-example
```

Load test is done with [wrk](https://github.com/wg/wrk), so it is required
to install it before testing. To start the load test:

```
./load_test.sh
```
