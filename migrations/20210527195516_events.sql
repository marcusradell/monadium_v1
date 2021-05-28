-- Single table for all events in the app.
create table if not exists events (
    -- Used to order events.
    sequence_num bigserial not null,
    -- ID for the events that belong together,
    -- It can be seen as an entity ID.
    stream_id uuid not null,
    -- Used to prevent concurrent updates on the same stream.
    version int not null,
    -- Event type, example: identities/new_member_created.
    type_ text not null,
    -- Event payload containing all the state changes.
    data jsonb not null,
    -- Event metadata that helps track the events through our system.
    meta jsonb not null,
    -- When the event got inserted into the DB.
    inserted_at timestamptz not null default now(),
    -- We primarily want to query the events in sequence.
    primary key (sequence_num),
    -- We block an update if it is based on an old stream version.
    unique (stream_id, version)
);

-- Helps us to query for all the events for a specific stream.
CREATE INDEX idx_event_stream_id ON events (stream_id);
