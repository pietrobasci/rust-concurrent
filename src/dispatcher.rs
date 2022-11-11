use std::sync::mpsc::{Sender, Receiver, channel};
use std::sync::Mutex;


pub struct Subscription<Msg: Clone> {
    rx: Receiver<Msg>,
}

impl<Msg: Clone> Subscription<Msg> {
    pub fn new(rx: Receiver<Msg>) -> Self {
        Subscription { rx: rx }
    }

    pub fn read(&self) -> Option<Msg> {
        match self.rx.recv() {
            Ok(msg) => return Some(msg),
            Err(_) => return None
        }
    }
}

struct State<Msg: Clone> {
    queue: Vec<Sender<Msg>>,
}

pub struct Dispatcher<Msg: Clone> {
    m: Mutex<State<Msg>>,
}

impl<Msg: Clone> Dispatcher<Msg> {
    pub fn new() -> Self {
        let s = State { queue: Vec::new() };
        Dispatcher { m: Mutex::new(s) }
    }

    pub fn dispatch(&self, msg: Msg) {
        let mut s = self.m.lock().unwrap();

        for i in (0..s.queue.len()).rev() {

            match s.queue[i].send(msg.clone()) {
                Ok(_) => (),
                Err(_) => { s.queue.remove(i);
                            println!("removed: {}", i); }
            }
        }
        println!("len: {}", s.queue.len());
    }

    pub fn subscribe(&self) -> Subscription<Msg> {
        let (sx,rx) = channel();

        let mut s = self.m.lock().unwrap();
        s.queue.push(sx);

        Subscription::new(rx)

    }

}