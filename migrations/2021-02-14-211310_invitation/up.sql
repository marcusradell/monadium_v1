-- Your SQL goes here
create table invitation (
    id UUID not null primary key,
    email varchar not null,
    created_at timestamp not null
);
