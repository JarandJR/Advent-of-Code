use std::{collections::HashMap, hash::Hash};

pub struct BiMap<Key, Value> {
    reverse: HashMap<Value, Key>,
    forward: HashMap<Key, Value>,
}

impl<K, V> BiMap<K, V> {
    pub fn new() -> Self {
        Self {
            forward: HashMap::new(),
            reverse: HashMap::new(),
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
    K: Eq + Hash + Clone,
    V: Eq + Hash + Clone,
{
    pub fn insert(&mut self, k: K, v: V) -> Option<(K, V)> {
        let value = self.forward.insert(k.clone(), v.clone());
        let key = self.reverse.insert(v, k);
        if value.is_none() || key.is_none() {
            return None;
        }
        Some((key.unwrap(), value.unwrap()))
    }

    pub fn get_by_key(&self, k: &K) -> Option<&V> {
        self.forward.get(k)
    }

    pub fn get_by_value(&self, v: &V) -> Option<&K> {
        self.reverse.get(v)
    }

    pub fn entry_forward(&mut self, key: K) -> Entry<'_, K, V> {
        if let Some(value) = self.forward.get(&key) {
            Entry::Occupied(OccupiedEntry {
                key,
                value: value.clone(),
                bimap_forward: &mut self.forward,
                bimap_reverse: &mut self.reverse,
            })
        } else {
            Entry::Vacant(VacantEntry {
                key,
                bimap_forward: &mut self.forward,
                bimap_reverse: &mut self.reverse,
            })
        }
    }

    pub fn entry_reverse(&mut self, value: V) -> Entry<'_, V, K> {
        if let Some(key) = self.reverse.get(&value) {
            Entry::Occupied(OccupiedEntry {
                key: value,
                value: key.clone(),
                bimap_forward: &mut self.reverse,
                bimap_reverse: &mut self.forward,
            })
        } else {
            Entry::Vacant(VacantEntry {
                key: value,
                bimap_forward: &mut self.reverse,
                bimap_reverse: &mut self.forward,
            })
        }
    }

    pub fn sync_forward(&mut self) {
        self.reverse.clear();
        self.forward.iter().for_each(|(k, v)| {
            self.reverse.insert(v.clone(), k.clone());
        });
    }

    pub fn sync_reverse(&mut self) {
        self.forward.clear();
        self.reverse.iter().for_each(|(v, k)| {
            self.forward.insert(k.clone(), v.clone());
        });
    }

    pub fn sync(&mut self, old_k: K, old_v: V) {
        let new_value = self.forward.remove(&old_k).unwrap();
        let new_key = self.reverse.remove(&old_v).unwrap();
        self.forward.insert(new_key.clone(), new_value.clone());
        self.reverse.insert(new_value, new_key);
    }

    pub fn forward_iter(&self) -> impl Iterator<Item = (&K, &V)> {
        self.forward.iter()
    }

    pub fn reverse_iter(&self) -> impl Iterator<Item = (&V, &K)> {
        self.reverse.iter()
    }
}

pub struct OccupiedEntry<'a, K, V> {
    key: K,
    value: V,
    bimap_forward: &'a mut HashMap<K, V>,
    bimap_reverse: &'a mut HashMap<V, K>,
}

pub struct VacantEntry<'a, K, V> {
    key: K,
    bimap_forward: &'a mut HashMap<K, V>,
    bimap_reverse: &'a mut HashMap<V, K>,
}

pub enum Entry<'a, K, V> {
    Occupied(OccupiedEntry<'a, K, V>),
    Vacant(VacantEntry<'a, K, V>),
}

impl<'a, K: Eq + std::hash::Hash + Clone, V: Eq + std::hash::Hash + Clone> OccupiedEntry<'a, K, V> {
    fn get_mut(self) -> (&'a mut K, &'a mut V) {
        (
            self.bimap_reverse.get_mut(&self.value).unwrap(),
            self.bimap_forward.get_mut(&self.key).unwrap(),
        )
    }
}

impl<'a, K: Eq + std::hash::Hash + Clone, V: Eq + std::hash::Hash + Clone> VacantEntry<'a, K, V> {
    fn insert(self, value: V) -> (&'a mut K, &'a mut V) {
        self.bimap_forward.insert(self.key.clone(), value.clone());
        self.bimap_reverse.insert(value.clone(), self.key.clone());
        (
            self.bimap_reverse.get_mut(&value).unwrap(),
            self.bimap_forward.get_mut(&self.key).unwrap(),
        )
    }
}

impl<'a, K: Eq + std::hash::Hash + Clone, V: Eq + std::hash::Hash + Clone> Entry<'a, K, V> {
    pub fn or_insert(self, default: V) -> (&'a mut K, &'a mut V) {
        match self {
            Entry::Occupied(occupied) => occupied.get_mut(),
            Entry::Vacant(vacant) => vacant.insert(default),
        }
    }
}

#[test]
fn test_functionality_of_map() {
    let mut map = BiMap::new();
    map.insert(1, "Hello");
    map.insert(2, "World");
    assert_eq!(map.get_by_key(&2), Some(&"World"));
    assert_eq!(map.get_by_value(&"Hello"), Some(&1));

    let (num, _) = map.entry_forward(3).or_insert("!");
    *num = 20;
    map.sync_reverse();

    assert_eq!(map.get_by_key(&3), None);
    assert_eq!(map.get_by_key(&20), Some(&"!"));

    let (_, word) = map.entry_forward(4).or_insert("#");
    *word = "¤";
    map.sync_forward();
    assert_eq!(map.get_by_value(&"#"), None);
    assert_eq!(map.get_by_value(&"¤"), Some(&4));

    let (num, word) = map.entry_forward(5).or_insert("&");
    let num_clone = *num;
    let word_clone = *word;
    *num = 10;
    *word = "?";
    map.sync(num_clone, word_clone);

    assert_eq!(map.get_by_key(&5), None);
    assert_eq!(map.get_by_key(&10), Some(&"?"));

    assert_eq!(map.get_by_value(&"&"), None);
    assert_eq!(map.get_by_value(&"?"), Some(&10));
}
