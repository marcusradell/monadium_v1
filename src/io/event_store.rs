use uuid::Uuid;

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Event {
    id: Uuid,
    sequence_num: u64,
    ttype: String,
}

type Error = Box<dyn std::error::Error>;
type Result<T> = std::result::Result<T, Error>;

pub trait EventStorer {
    fn add(&mut self, event: Event) -> Result<()>;

    fn list(&self) -> Result<Vec<Event>>;
}

pub struct EventStoreMock {
    data: Vec<Event>,
}

impl EventStorer for EventStoreMock {
    fn add(&mut self, event: Event) -> Result<()> {
        self.data.push(event);
        Ok(())
    }

    fn list(&self) -> Result<Vec<Event>> {
        Ok(self.data.to_owned())
    }
}

#[allow(dead_code)]
impl EventStoreMock {
    fn new() -> Self {
        EventStoreMock { data: vec![] }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn add() {
        let event = Event {
            id: Uuid::from_u128(1),
            sequence_num: 1,
            ttype: "DO_THING".into(),
        };

        let mut store = EventStoreMock::new();
        store.add(event.clone()).unwrap();

        let result = store.list().unwrap();

        assert_eq!(result, vec![event]);
    }
}
