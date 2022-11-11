use std::collections::HashMap;
use std::thread::ThreadId;
use std::thread;
use std::sync::{Mutex, Condvar};

struct State {
    values: HashMap<ThreadId,f32>,
    full: bool,
    num: usize,
}

pub struct Joiner {
    n_threads: usize,
    m: Mutex<State>,
    cv: Condvar,
}

impl Joiner {
    pub fn new(n_threads: usize) -> Self {
        let s = State {values: HashMap::new(), full: false, num: 0};
        Joiner {n_threads: n_threads, m: Mutex::new(s), cv: Condvar::new()}
    }

    pub fn supply(&self, value: f32) -> HashMap<ThreadId,f32> {
        let mut s = self.m.lock().unwrap();

        while s.full {
            s = self.cv.wait(s).unwrap();
        }

        s.values.insert(thread::current().id(), value);
        s.num += 1;

        if s.num < self.n_threads {
            while !s.full {
                s = self.cv.wait(s).unwrap();
            }
        } else {
            s.full = true;
            self.cv.notify_all();
        }

        let res = s.values.clone();
        s.num -= 1;

        if s.num == 0 {
            s.values.clear();
            s.full = false;
            self.cv.notify_all();
        }

        res

    }

}