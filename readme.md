# Monadium

An extremely modular web service written in Rust!

## Structure

The source code is split into two modules; `domain` and `io`.

### Io

This module contains every input/output-based driver you might need to run your service. This will eventually be broken out to its own crate, making it really simple to split up your service into multiple smaller services in the future.

It also contains the `error` module that should be used by all other `io` modules so that error handling is simple. The `error` module implements `actix-web`s `ResponseError`, making it possible to use the error results directly in actix.

### Domain

Contains all of the business logic which uses the `io` modules.
The structure of the domains will be based on event sourcing, which uses an append-only table with reducer functions to replay the actual state as needed.

- `health` will be extracted into the future crate as a shared app for every service that needs `live` and `ready` checks.

- `identities` takes care of signing up and signing in.

## Planning

Check the project board here on GitHub. It strives to be a user story map with columns for each feature and milestones for each release across features.

## Developing

We use trunk-based development, so there are no PRs in most cases. Don't feel afraid to break the rules though until you feel comfortable.

- Copy/paste `.env.example` as `.env`. Populate the values as you want.

- `cargo run` will run the app locally.

- `cargo install cargo-watch` will enable you to run `cargo watch -x run`, and is the preferred way if you prefer to not TDD.

## Testing

- TODO. Some code for testing might exist, but will be changed.

## Deploying

This repo takes care of the Continuous Integration pipeline, and then pushes a commit to the `monadium-infra` repo which then deploys to production.
There's no staging environment, and any such environment should be thrown away after it has been used to ensure that setting up new environments is simple and automated.
