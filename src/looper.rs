use std::sync::mpsc::{channel, Sender};
use std::thread;

#[derive(Clone)]
pub struct Looper<Message: Send + 'static> {
    sender: Sender<Message>
}

impl<Message: Send + 'static> Looper<Message> {
    pub fn new(process: fn(msg: Message)->(), cleanup: fn()->()) -> Self {
        let (sx, rx) = channel();

        thread::spawn( move || {
            loop {
                match rx.recv() {
                    Ok(msg) => process(msg),
                    Err(_) => break
                }
            }
            cleanup();
        } );

        Looper { sender: sx }
    }

    pub fn send(&self, msg: Message) -> () {
        self.sender.send(msg).unwrap();
    }
}