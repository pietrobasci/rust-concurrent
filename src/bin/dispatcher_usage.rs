use rust_concurrent::dispatcher::Dispatcher;
use std::sync::Arc;
use std::thread;
use std::thread::sleep;
use std::time::Duration;

fn main() {
    let dispatcher = Arc::new(Dispatcher::new());
    let dispatcher1 = Arc::clone(&dispatcher);
    let dispatcher2 = Arc::clone(&dispatcher);
    let dispatcher3 = Arc::clone(&dispatcher);

    let t1 = thread::spawn(move || {
        let res = dispatcher.subscribe();
        for _ in 0..3 {
            let msg = res.read();
            if msg.is_some() { println!("T1: {}", msg.unwrap()); }
        }
    });

    let t2 = thread::spawn(move || {
        let res = dispatcher1.subscribe();
        for _ in 0..3 {
            let msg = res.read();
            if msg.is_some() { println!("T2: {}", msg.unwrap()); }
        }
    });

    let t3 = thread::spawn(move || {
        let res = dispatcher2.subscribe();
        //for _ in 0..3 {
        let msg = res.read();
        if msg.is_some() { println!("T3: {}", msg.unwrap()); }
        //}
    });

    let t4 = thread::spawn(move || {
        for _ in 0..3 {
            println!("Sending..");
            sleep(Duration::from_secs(2));
            dispatcher3.dispatch("hello");
        }
    });


    println!("Done");

    t1.join().unwrap();
    t2.join().unwrap();
    t3.join().unwrap();
    t4.join().unwrap();

}