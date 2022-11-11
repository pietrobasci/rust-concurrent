use std::sync::{Mutex, Condvar};

struct State<T> {
    val1: Option<T>,
    val2: Option<T>,
}

pub struct Exchanger<T> where T: Clone {
    m: Mutex<State<T>>,
    cv: Condvar,
}

impl<T> Exchanger<T> where T: Clone {

    pub fn new() -> Self {
        let state = State { val1: None, val2: None };
        Exchanger { m: Mutex::new(state), cv: Condvar::new() }
    }

    pub fn exchange(&self, t: T) -> T {
        let mut s = self.m.lock().unwrap();

        while s.val1.is_some() && s.val2.is_some() {
            s = self.cv.wait(s).unwrap();
        }

        let mut res = None;

        if s.val1.is_none() {
            s.val1 = Some(t);
            while s.val2.is_none() {
                s = self.cv.wait(s).unwrap();
            }
            res = s.val2.clone();

            //drop(s);
            //sleep(Duration::new(1, 0));
            //s = self.m.lock().unwrap();

            s.val1 = None;
            s.val2 = None;
            self.cv.notify_all();
        } else if s.val2.is_none() {
            s.val2 = Some(t);
            res = s.val1.clone();
            self.cv.notify_one();
        }

        res.unwrap()
    }
}