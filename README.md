crawlcomply-backend
===================
[![License](https://img.shields.io/badge/license-Apache--2.0%20OR%20MIT-blue.svg)](https://opensource.org/licenses/Apache-2.0)

Backend implementation handling models and routes for `Model`s, `Profile`s, and more.

Server frontend can be found at parent repository: https://github.com/replica-ml/serve-replica (clone this one directory
above for that to build)

## Development guide

### Diesel

[Diesel](https://diesel.rs) is the most popular Rust database abstraction for PostgreSQL, MySQL, and SQLite.

Most of this guide is taken from https://diesel.rs/guides/getting-started @ [
`6fbfd94`](https://github.com/sgrif/diesel.rs-website/blob/6fbfd94/src/guides/getting-started.md).

#### Dependencies

```sh
$ cargo install --force cargo-binstall dsync
$ cargo binstall diesel_cli
$ printf '%s\n' \
  DATABASE_URL='postgres://rest_user:rest_pass@localhost/rest_db' \
  REDIS_URL='redis://127.0.0.1/' > .env
```

##### Initial setup

If no migrations directory exists, run:

```sh
$ diesel setup
```

#### Migrations (e.g., new table)

```sh
$ diesel migration generate create_profile
```

#### Run migrations

```sh
$ [ -d '../rust-actix-diesel-auth-scaffold' ] || \
  git clone --depth=1 https://github.com/offscale/rust-actix-diesel-auth-scaffold ../rust-actix-diesel-auth-scaffold
$ diesel migration run --migration-dir ../rust-actix-diesel-auth-scaffold/migrations
$ diesel migration run
```

#### Test that rollback works

```sh
$ diesel migration redo
```

#### Generate associated `struct`s (like the `class` ORMs of other languages)

```sh
$ dsync -c 'diesel::pg::PgConnection' -d 'utoipa::ToSchema' -i 'src/schema.rs' -o 'src/models'
```

## Docker usage

Install Docker, and then run the following, which will make a server available at http://localhost:3000:

```sh
$ docker compose up
````

NOTE: You may need to configure this for your architecture first, for example:

```sh
$ docker compose build --build-arg ARCH_VARIANT='amd64' \
                       --build-arg ARCH='x86_64'
$ docker compose up
```

Or to work with just one image and provide your own database and redis:

```sh
$ docker build -f 'debian.Dockerfile' -t "${PWD##*/}"':latest' .
$ docker run -e DATABASE_URL="$DATABASE_URL" \
             -e REDIS_URL='localhost:6379' \
             -p '3000:3000' \
             --name 'serve_api' \
             "${PWD##*/}"
```

## Native Usage

Install Rust, `git`, and ensure you have your PostgreSQL and Redis/Valkey services setup.

### PostgreSQL

One way to install PostgreSQL is with my cross-platform https://github.com/SamuelMarks/libscript:

```sh
$ [ -d /tmp/libscript ] || git clone --depth=1 --single-branch https://github.com/SamuelMarks/libscript /tmp/libscript
$ env -i HOME="$HOME" \
         PATH="$PATH" \
         POSTGRES_USER='rest_user' \
         POSTGRES_SERVICE_PASSWORD='addGoodPasswordhere' \
         POSTGRES_PASSWORD='rest_pass' \
         POSTGRES_HOST='localhost' \
         POSTGRES_DB='rest_db' \
         '/tmp/libscript/_lib/_storage/postgres/setup.sh'
```

(on Windows use `set` and `libscript\_lib\_storage\postgres\setup.cmd`)

### Valkey (Redis-compatible)

One way to install the Redis-compatible Valkey is with my cross-platform https://github.com/SamuelMarks/libscript:

```sh
$ [ -d '/tmp/libscript' ] || git clone --depth=1 --single-branch https://github.com/SamuelMarks/libscript /tmp/libscript
$ env -i HOME="$HOME" \
         PATH="$PATH" \
         '/tmp/libscript/_lib/_storage/valkey/setup.sh'
```

(on Windows use Garnet: https://github.com/microsoft/garnet)

### Environment setup

Add an `.env` file or otherwise add these environment variables; replacing connection strings with what you use:

```sh
DATABASE_URL=postgres://rest_user:rest_pass@localhost/rest_db
REDIS_URL=redis://127.0.0.1/
```

### Test

```sh
$ cargo test
```

## Contribution guide

Ensure all tests are passing [`cargo test`](https://doc.rust-lang.org/cargo/commands/cargo-test.html) and [
`rustfmt`](https://github.com/rust-lang/rustfmt) has been run. This can be with [
`cargo make`](https://github.com/sagiegurari/cargo-make); installable with:

```sh
$ cargo install --force cargo-make
```

Then run:

```sh
$ cargo make
```

Finally, we recommend [feature-branches](https://martinfowler.com/bliki/FeatureBranch.html) with an
accompanying [pull-request](https://docs.github.com/en/pull-requests/collaborating-with-pull-requests/proposing-changes-to-your-work-with-pull-requests/about-pull-requests).
</small>

<hr/>

## License

Licensed under either of:

- Apache License, Version 2.0 ([LICENSE-APACHE](LICENSE-APACHE) or <https://www.apache.org/licenses/LICENSE-2.0>)
- MIT license ([LICENSE-MIT](LICENSE-MIT) or <https://opensource.org/licenses/MIT>)

at your option.

### Contribution

Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in the work by you, as defined in the Apache-2.0 license, shall be
licensed as above, without any additional terms or conditions.
