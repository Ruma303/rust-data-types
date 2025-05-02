pub mod hashset {
    use std::collections::HashSet;

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