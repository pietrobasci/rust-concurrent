use std::sync::{Mutex, Condvar};

struct State {
    count: i32,
}

pub struct ExecutionLimiter {
    n: i32,
    m: Mutex<State>,
    cv: Condvar
}

impl ExecutionLimiter {
    pub fn new(n:i32) -> Self {
        let state = State {count: 0};
        ExecutionLimiter {n:n, m: Mutex::new(state), cv: Condvar::new()}
    }

    pub fn execute<R>(&self, f: fn()->R) -> R {
        let mut s = self.m.lock().unwrap();

        while s.count == self.n {
            s = self.cv.wait(s).unwrap();
        }

        s.count += 1;

        drop(s);
        let res = f();
        s = self.m.lock().unwrap();

        s.count -= 1;
        self.cv.notify_one();

        res

    }
}