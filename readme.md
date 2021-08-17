# Monadium

An extremely modular web service written in Rust!

## Structure

The source code is split into two modules; `domain` and `io`.

### Io

This module contains every input/output-based driver you might need to run your service. This will eventually be broken out to its own crate, making it really simple to split up your service into multiple smaller services in the future.

It also contains the `error` module that should be used by all other `io` modules so that error handling is simple. The `error` module implements `actix-web`s `ResponseError`, making it possible to use the error results directly in actix.

### Domain

Contains all of the business logic which uses the `io` modules.

`health` will be extracted into the future crate as a shared app for every service that needs `live` and `ready` checks.

`identities` takes care of signing up and signing in.
