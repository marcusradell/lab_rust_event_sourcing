use super::State;

#[derive(Debug)]
pub struct Event {
    pub name: String,
}

pub fn fold(s: State, e: Event) -> State {
    State {
        id: s.id,
        name: e.name,
    }
}
