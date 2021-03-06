-- Your SQL goes here
create table identity (
    id serial primary key,
    email varchar unique not null,
    password_hash varchar not null,
    role varchar not null,
    created_at timestamp not null
);
