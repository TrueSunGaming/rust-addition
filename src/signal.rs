use std::collections::HashMap;

pub struct Signal<'a, T> {
    listeners: Vec<&'a dyn Fn(&'a T)>,
    index_map: HashMap<&'static str, usize>
}

impl<'a, T> Signal<'a, T> {
    pub fn new() -> Self {
        Self {
            listeners: Vec::new(),
            index_map: HashMap::new()
        }
    }

    pub fn connect(&mut self, callback: &'a dyn Fn(&'a T), name: Option<&'static str>) {
        self.listeners.push(callback);
    
        match name {
            Some(v) => {
                self.index_map.insert(v, self.listeners.len() - 1);
            }

            _ => ()
        };
    }

    pub fn disconnect(&mut self, name: &str) {
        let index = *self.index_map.get(name).unwrap();

        self.listeners.remove(index);
        self.index_map.remove(name);

        for i in &mut self.index_map {
            if *i.1 > index {
                *i.1 -= 1;
            }
        }
    }

    pub fn emit(&self, data: &'a T) {
        for i in self.listeners.clone() {
            i(data);
        }
    }
}