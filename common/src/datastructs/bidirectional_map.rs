/*
use std::{
    collections::{hash_map::Entry, HashMap},
    hash::Hash,
};
pub struct BiMap<K, V> {
    key_map: HashMap<V, K>,
    value_map: HashMap<K, V>,
}

impl<K, V> BiMap<K, V> {
    pub fn new() -> Self {
        Self {
            key_map: HashMap::new(),
            value_map: HashMap::new(),
        }
    }
}

impl<K, V> Default for BiMap<K, V> {
    fn default() -> Self {
        Self::new()
    }
}

impl<K, V> BiMap<K, V>
where
    K: Hash + PartialEq + Eq + Clone,
    V: Hash + PartialEq + Eq + Clone,
{
    pub fn insert(&mut self, k: K, v: V) -> (Option<K>, Option<V>) {
        let key_opt = self.key_map.insert(v.clone(), k.clone());
        let value_opt = self.value_map.insert(k, v);
        if key_opt.is_none() || value_opt.is_none() {
            return (None, None);
        }
        (key_opt, value_opt)
    }

    pub fn get_key(&self, v: &V) -> Option<&K> {
        self.key_map.get(v)
    }

    pub fn get_value(&self, k: &K) -> Option<&V> {
        self.value_map.get(k)
    }

    pub fn contains_key(&self, k: &K) -> bool {
        self.value_map.contains_key(k)
    }

    pub fn contains_value(&self, v: &V) -> bool {
        self.key_map.contains_key(v)
    }

    pub fn entry_key(&mut self, v: &V) -> Entry<V, K> {
        self.key_map.entry(v.clone())
    }

    pub fn entry_value(&mut self, k: &K) -> Entry<K, V> {
        self.value_map.entry(k.clone())
    }

    pub fn entry_k(&mut self, key: K) -> BiMapEntry<K, V> {
        match self.value_map.entry(key.clone()) {
            Entry::Occupied(occupied) => BiMapEntry::Occupied(BiOccupiedEntry {
                key: self.key_map.get(occupied.get()).unwrap().clone(),
                value: occupied.get().clone(),
                key_map: &mut self.key_map,
                value_map: &mut self.value_map,
            }),
            Entry::Vacant(_) => BiMapEntry::Vacant(BiVacantEntry {
                key,
                key_map: &mut self.key_map,
                value_map: &mut self.value_map,
            }),
        }
    }
}

pub enum BiMapEntry<'a, K, V> {
    Occupied(BiOccupiedEntry<'a, K, V>),
    Vacant(BiVacantEntry<'a, K, V>),
}

impl<'a, K, V> BiMapEntry<'a, K, V>
where
    K: Clone + Eq + Hash,
    V: Clone + Eq + Hash,
{
    pub fn or_insert(self, value: V) -> &'a mut V {
        match self {
            BiMapEntry::Occupied(mut entry) => entry.get_mut(),
            BiMapEntry::Vacant(entry) => entry.insert(value),
        }
    }
}


pub struct BiOccupiedEntry<'a, K, V> {
    key: K,
    value: V,
    key_map: &'a mut HashMap<V, K>,
    value_map: &'a mut HashMap<K, V>,
}

impl<'a, K, V> BiOccupiedEntry<'a, K, V>
where
    K: Clone + Eq + Hash,
    V: Clone + Eq + Hash,
{
    pub fn get_mut(&mut self) -> &mut V {
        self.value_map.get_mut(&self.key).unwrap()
    }

    pub fn insert(&mut self, new_value: V) -> V {
        let old_value = self.value_map.insert(self.key.clone(), new_value.clone()).unwrap();
        self.key_map.remove(&old_value);
        self.key_map.insert(new_value, self.key.clone());
        old_value
    }
}

pub struct BiVacantEntry<'a, K, V> {
    key: K,
    value_map: &'a mut HashMap<K, V>,
    key_map: &'a mut HashMap<V, K>,
}

impl<'a, K, V> BiVacantEntry<'a, K, V>
where
    K: Clone + Eq + Hash,
    V: Clone + Eq + Hash,
{
    pub fn insert(self, value: V) -> &'a mut V {
        self.key_map.insert(value.clone(), self.key.clone());
        self.value_map.insert(self.key, value).unwrap()
    }
}
*/
