#[derive(Debug)]
pub struct Event {
    pub name: String,
}

pub fn fold(s: super::State, e: Event) -> super::State {
    super::State {
        id: s.id,
        name: e.name,
    }
}
