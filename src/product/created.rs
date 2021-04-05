use super::State;

#[derive(Debug)]
pub struct Event {
    pub id: String,
    pub name: String,
}

pub fn fold(e: Event) -> State {
    State {
        id: e.id,
        name: e.name,
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn fold_succeeded() {
        let expected = State {
            id: "1".into(),
            name: "Doggie".into(),
        };

        let result = fold(Event {
            id: "1".into(),
            name: "Doggies".into(),
        });

        assert_eq!(result, expected);
    }
}
