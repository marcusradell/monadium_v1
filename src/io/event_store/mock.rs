use std::convert::TryInto;

use chrono::TimeZone;
use sqlx::types::Json;

use super::types::{Event, EventStorer};
use crate::io::{event_store::types::EventMeta, result::Result};

pub struct EventStoreMock<T: Clone> {
    store: Vec<Event<T>>,
    counter: u32,
}

impl<T: Clone> EventStorer<T> for EventStoreMock<T> {
    fn add(
        &mut self,
        event_type: &str,
        version: i64,
        stream_id: uuid::Uuid,
        data: T,
        cid: uuid::Uuid,
    ) -> Result<()> {
        let event = Event {
            sequence_num: self
                .counter
                .try_into()
                .expect("Failed to use EventStoreMock.counter as an Event.sequence_num."),
            inserted_at: chrono::Utc.ymd(1999, 06, 01).and_hms(23, 55, self.counter),
            event_type: event_type.into(),
            version,
            data: Json(data),
            meta: Json(EventMeta { cid }),
            stream_id,
        };

        self.store.push(event);

        self.counter += 1;

        Ok(())
    }

    fn list(&self) -> Result<Vec<Event<T>>> {
        Ok(self.store.to_owned())
    }
}

#[allow(dead_code)]
impl<T: Clone> EventStoreMock<T> {
    fn new() -> Self {
        EventStoreMock {
            store: vec![],
            counter: 1,
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::io::event_store::types::EventMeta;
    use sqlx::types::Json;
    use uuid::Uuid;

    #[test]
    fn add() {
        let event: Event<String> = Event {
            sequence_num: 1,
            event_type: "DO_THING".into(),
            stream_id: Uuid::from_u128(1),
            meta: Json(EventMeta {
                cid: Uuid::from_u128(2),
            }),
            data: Json("data".into()),
            version: 1,
            inserted_at: chrono::Utc.ymd(1999, 06, 01).and_hms(23, 55, 1),
        };

        let mut store = EventStoreMock::new();

        store
            .add(
                "DO_THING",
                1,
                Uuid::from_u128(1),
                "data".into(),
                Uuid::from_u128(2),
            )
            .unwrap();

        let result = store.list().unwrap();

        assert_eq!(result, vec![event]);
    }
}
