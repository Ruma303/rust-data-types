use std::collections::HashSet;

pub fn run() {
    let mut set = HashSetWrapper::new();
    set.insert(1);
    set.insert(2);
    set.insert(3);

    if set.contains(&2) {
        println!("Found: 2");
    } else {
        println!("Not found");
    }

    set.print();
}

pub struct HashSetWrapper<T> {
    set: HashSet<T>,
}

impl<T> HashSetWrapper<T>
where
    T: std::hash::Hash + Eq,
{
    pub fn new() -> Self {
        HashSetWrapper {
            set: HashSet::new(),
        }
    }

    pub fn insert(&mut self, value: T) {
        self.set.insert(value);
    }

    pub fn contains(&self, value: &T) -> bool {
        self.set.contains(value)
    }
}
impl<T> HashSetWrapper<T>
where
    T: std::hash::Hash + Eq + std::fmt::Debug,
{
    pub fn print(&self) {
        for value in &self.set {
            println!("{:?}", value);
        }
    }
}
