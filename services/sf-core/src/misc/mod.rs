use std::collections::HashMap;

pub struct BiMap<K, V> {
    forward: HashMap<K, V>,
    reverse: HashMap<V, K>
}

impl<K, V> BiMap<K, V> 
where
    K: Eq + std::hash::Hash + Clone,
    V: Eq + std::hash::Hash + Clone,
{
    pub fn new() -> Self {
        BiMap {
            forward: HashMap::new(),
            reverse: HashMap::new(),
        }
    }

    pub fn insert(&mut self, key: K, value: V) {
        self.forward.insert(key.clone(), value.clone());
        self.reverse.insert(value, key);
    }

    pub fn get_forward(&self, key: &K) -> Option<&V> {
        self.forward.get(key)
    }

    pub fn get_reverse(&self, value: &V) -> Option<&K> {
        self.reverse.get(value)
    }

    pub fn iter_forward(&self) -> std::collections::hash_map::Iter<'_, K, V> {
        self.forward.iter()
    }
    
    pub fn iter_reverse(&self) -> std::collections::hash_map::Iter<'_, V, K> {
        self.reverse.iter()
    }
}