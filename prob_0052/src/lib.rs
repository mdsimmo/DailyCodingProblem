/*
Implement an LRU (Least Recently Used) cache. It should be able to be initialized with a cache size n, and contain the following methods:

    set(key, value): sets key to value. If there are already n items in the cache and we are adding a new item, then it should also remove the least recently used item.
    get(key): gets the value at key. If no such key exists, return null.

Each operation should run in O(1) time.

Time to complete: 40 min
*/

extern crate linked_hash_set;
use linked_hash_set::LinkedHashSet;

use std::collections::HashMap;
use std::hash::Hash;

#[derive(Debug)]
pub struct Cache<K, V> where K: Eq+Hash+Copy {
    size: usize,
    map: HashMap<K, V>,
    inserts: LinkedHashSet<K>,
}

impl <K, V> Cache<K, V> where K: Eq+Hash+Copy {
    pub fn new(size: usize) -> Self {
        assert!(size > 0);
        Cache {
            size,
            map: HashMap::with_capacity(size),
            inserts: LinkedHashSet::with_capacity(size)
        }
    }

    pub fn put(&mut self, key: K, value: V) {
        self.map.insert(key, value);
        self.inserts.insert(key);
        if self.map.len() > self.size {
            let first_entry = self.inserts.pop_front().unwrap();
            self.map.remove(&first_entry);
        }
    }

    pub fn get(&self, key: K) -> Option<&V> {
        self.map.get(&key)
    }
}

#[cfg(test)]
mod tests {
    use Cache;

    #[test]
    fn test_basic_expire() {
        let mut cache = Cache::new(3);
        cache.put("1", 1);
        cache.put("2", 2);
        cache.put("3", 3);
        cache.put("4", 4);

        assert_eq!(Some(&2), cache.get("2"));
        assert_eq!(Some(&3), cache.get("3"));
        assert_eq!(Some(&4), cache.get("4"));

        assert_eq!(None, cache.get("1"));
        assert_eq!(None, cache.get("5"));
    }

    #[test]
    fn test_multiple_insert() {
        let mut cache = Cache::new(3);
        cache.put("1", 1);
        cache.put("2", 2);
        cache.put("1", 11);
        cache.put("3", 3);
        cache.put("4", 4);

        assert_eq!(Some(&11), cache.get("1"));
        assert_eq!(None, cache.get("2"));
        assert_eq!(Some(&3), cache.get("3"));
        assert_eq!(Some(&4), cache.get("4"));
    }
}