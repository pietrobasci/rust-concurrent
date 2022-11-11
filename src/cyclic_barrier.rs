use std::sync::{Mutex, Condvar};

struct State {
    num: u8,
    full: bool,
}

pub struct CyclicBarrier {
    m: Mutex<State>,
    cv: Condvar,
    max_num: u8,
}

impl CyclicBarrier {

    pub fn new(max_num: u8) -> Self {
        let s = State {num: 0, full: false};
        CyclicBarrier { m: Mutex::new(s), cv: Condvar::new(), max_num: max_num}
    }

    pub fn wait(&self) -> () {

        let mut s = self.m.lock().unwrap();

        while s.full {
            s = self.cv.wait(s).unwrap();
        }

        s.num += 1;

        if s.num < self.max_num {
            while !s.full {
                s = self.cv.wait(s).unwrap();
            }
        } else {
            s.full = true;
            self.cv.notify_all();
        }

        s.num -= 1;

        if s.num == 0 {
            s.full = false;
            self.cv.notify_all();
        }

    }

}