use std::sync::{Arc, Mutex, Condvar};
use std::collections::HashMap;
use std::hash::Hash;
use crate::f_cache::EntryState::{Present, Pending};
use std::collections::hash_map::Entry::{Occupied, Vacant};

#[derive(Eq, PartialEq)]
enum EntryState<V> {
    Pending,
    Present(Arc<V>)
}

pub struct FCache<K:Eq+Hash+Clone,V> {
    m: Mutex<HashMap<K, EntryState<V>>>,
    cv: Condvar,
}

impl<K:Eq+Hash+Clone,V> FCache<K,V> {
    pub fn new() -> Self {
        FCache {m: Mutex::new(HashMap::new()), cv: Condvar::new()}
    }

    pub fn get(&self, k:K, f:impl FnOnce(&K) -> V + 'static) -> Arc<V> {
        let mut map = self.m.lock().unwrap();
        loop {
            match map.entry(k.clone()) {
                Occupied(e) => {
                    match e.get() {
                        Pending => {
                            map = self.cv.wait_while(
                                map,
                                |map| {
                                    if let Occupied(e) = map.entry(k.clone()) {
                                        if let Present(..) = e.get() { return false }
                                    }
                                    true
                                }).unwrap();
                        }
                        Present(v) => {
                            return v.clone();
                        }
                    }
                }
                Vacant(e) => {
                    e.insert(Pending);
                    drop(map);
                    let v = Arc::new(f(&k));
                    map = self.m.lock().unwrap();
                    map.entry(k.clone())
                        .and_modify(|e| {*e = Present(v.clone())});
                    self.cv.notify_all();
                    return v;
                }
            }
        }
    }
}