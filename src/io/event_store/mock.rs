use super::types::{Event, EventStorer};
use crate::io::result::Result;

pub struct EventStoreMock<T: Clone> {
    data: Vec<Event<T>>,
}

impl<T: Clone> EventStorer<T> for EventStoreMock<T> {
    fn add(&mut self, event: Event<T>) -> Result<()> {
        self.data.push(event);
        Ok(())
    }

    fn list(&self) -> Result<Vec<Event<T>>> {
        Ok(self.data.to_owned())
    }
}

#[allow(dead_code)]
impl<T: Clone> EventStoreMock<T> {
    fn new() -> Self {
        EventStoreMock { data: vec![] }
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use sqlx::types::Json;
    use uuid::Uuid;

    #[test]
    fn add() {
        let event: Event<String> = Event {
            stream_id: Uuid::from_u128(1),
            sequence_num: 1,
            event_type: "DO_THING".into(),
            cid: Uuid::from_u128(100),
            data: Json("data".into()),
            version: 1,
            inserted_at: chrono::Utc::now(),
        };

        let mut store = EventStoreMock::new();
        store.add(event.clone()).unwrap();

        let result = store.list().unwrap();

        assert_eq!(result, vec![event]);
    }
}
