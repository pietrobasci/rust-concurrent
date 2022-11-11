use std::sync::{Mutex, Condvar};

pub struct CountDownLatch {
    m: Mutex<usize>,
    cv: Condvar
}

impl CountDownLatch {
    pub fn new(count: usize) -> Self {
        CountDownLatch { m: Mutex::new(count), cv: Condvar::new() }
    }

    pub fn count_down(&self) {
        let mut s = self.m.lock().unwrap();
        if *s == 0 { panic!("Count is already 0") }
        *s -= 1;
        if *s == 0 {
            self.cv.notify_all();
        }
    }

    pub fn wait(&self) {
        let mut s = self.m.lock().unwrap();
        while *s > 0 {
            s = self.cv.wait(s).unwrap();
        }
    }

}