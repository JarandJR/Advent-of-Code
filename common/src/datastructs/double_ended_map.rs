use std::{collections::HashMap, hash::Hash};

pub struct DoubleEndedMap<K, V> {
    key_map: HashMap<V, K>,
    value_map: HashMap<K, V>,
}

impl<K, V> DoubleEndedMap<K, V>
where
    K: Hash + PartialEq + Eq + Clone,
    V: Hash + PartialEq + Eq + Clone,
{
    pub fn new() -> Self {
        Self {
            key_map: HashMap::new(),
            value_map: HashMap::new(),
        }
    }

    pub fn insert(&mut self, k: K, v: V) -> Option<(K, V)> {
        let key_opt = self.key_map.insert(v.clone(), k.clone());
        let value_opt = self.value_map.insert(k, v);
        if key_opt.is_none() || value_opt.is_none() {
            return None;
        }
        Some((key_opt.unwrap(), value_opt.unwrap()))
    }

    pub fn get_key(&self, v: &V) -> Option<&K> {
        self.key_map.get(v)
    }

    pub fn get_value(&self, k: &K) -> Option<&V> {
        self.value_map.get(k)
    }
}

/*
impl<'a, K: 'a, V: 'a> DoubleEndedMap<K, V>
where
    &'a K: Hash + PartialEq + Eq,
    &'a V: Hash + PartialEq + Eq,
{
    pub fn new() -> Self {
        Self { key_map: HashMap::new(), value_map: HashMap::new() }
    }

    pub fn insert(&mut self, k: &'a K, v: &'a V) -> Option<(K, V)>{
        let key_opt = self.key_map.insert(v, k);
        let value_opt = self.value_map.insert(k, v);
        if key_opt.is_none() || value_opt.is_none() {
            return None;
        }
        Some((key_opt.unwrap(), value_opt.unwrap()))
    }
}*/
