use std::collections::HashSet;

#[derive(Clone, Default)]
pub struct AppState {
    pub data: HashSet<String>,
}

impl AppState {
    pub fn add(&mut self, value: String) {
        self.data.insert(value);
    }

    pub fn contains(&self, value: &String) -> bool {
        self.data.contains(value)
    }
}
