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

As far as the service create file for the table **person.tbl**, it is
necessary to clean it before each load test run. So use this command to start:

```
cargo build --release && rm -f person.tbl && ROCKET_ENV=prod ./target/release/mytable-rest-example
```

Load test is written in Python 3 with the library
[requests](https://pypi.org/project/requests/), so it is needed to install it
before execute. To start load test:

```
python load_test.py
```
