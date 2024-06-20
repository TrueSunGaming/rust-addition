use std::collections::HashMap;

pub struct Factory<T> {
    map: HashMap<&'static str, T>
}

impl<T> Factory<T> {
    pub fn new() -> Self {
        Self {
            map: HashMap::new()
        }
    }

    pub fn register(&mut self, key: &'static str, value: T) {
        self.map.insert(key, value);
    }

    pub fn generate(&self, key: &'static str) -> Option<&T> {
        self.map.get(key)
    }
}