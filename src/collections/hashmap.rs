pub mod hashmap {
    use std::collections::HashMap;

    pub fn run() { // main posso usarlo?
        let mut map = HashMapWrapper::new();
        map.insert("key1", 1);
        map.insert("key2", 2);
        map.insert("key3", 3);

        if let Some(value) = map.get(&"key2") {
            println!("Found: {}", value);
        } else {
            println!("Not found");
        }

        map.print();
    }

    pub struct HashMapWrapper<K, V> {
        map: HashMap<K, V>,
    }

    impl<K, V> HashMapWrapper<K, V>
    where
        K: std::hash::Hash + Eq,
    {
        pub fn new() -> Self {
            HashMapWrapper {
                map: HashMap::new(),
            }
        }

        pub fn insert(&mut self, key: K, value: V) {
            self.map.insert(key, value);
        }

        pub fn get(&self, key: &K) -> Option<&V> {
            self.map.get(key)
        }
    }
    impl<K, V> HashMapWrapper<K, V>
    where
        K: std::hash::Hash + Eq + std::fmt::Debug,
        V: std::fmt::Debug,
    {
        pub fn print(&self) {
            for (key, value) in &self.map {
                println!("{:?}: {:?}", key, value);
            }
        }
    }
}