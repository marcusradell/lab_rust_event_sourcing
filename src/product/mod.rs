pub mod created;
pub mod name_changed;

#[derive(Debug, PartialEq, Eq)]
pub struct State {
    pub id: String,
    pub name: String,
}
