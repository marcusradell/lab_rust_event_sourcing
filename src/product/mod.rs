pub mod created;
pub mod name_changed;

#[derive(Debug)]
pub struct State {
    pub id: String,
    pub name: String,
}
