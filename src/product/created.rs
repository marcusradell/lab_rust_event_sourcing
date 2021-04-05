#[derive(Debug)]
pub struct Event {
    pub id: String,
    pub name: String,
}

pub fn fold(e: Event) -> super::State {
    super::State {
        id: e.id,
        name: e.name,
    }
}
