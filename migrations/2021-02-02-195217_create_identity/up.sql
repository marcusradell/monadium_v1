-- Your SQL goes here
create table identity (
    id serial primary key,
    email varchar not null,
    password_hash varchar not null
)