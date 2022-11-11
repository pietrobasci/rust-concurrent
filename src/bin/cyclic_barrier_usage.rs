use rust_concurrent::cyclic_barrier::CyclicBarrier;
use std::sync::Arc;
use std::thread;
use std::thread::sleep;
use std::time::Duration;

fn main() {
    let cdl = Arc::new(CyclicBarrier::new(3));
    let cdl1 = Arc::clone(&cdl);
    let cdl2 = Arc::clone(&cdl);

    let t1 = thread::spawn(move || {
        sleep(Duration::from_secs(2));
        cdl1.wait();
        println!("Thread1 done!");
    });

    let t2 = thread::spawn(move || {
        sleep(Duration::from_secs(3));
        cdl2.wait();
        println!("Thread2 done!");
    });

    cdl.wait();
    println!("Done");

    t1.join().unwrap();
    t2.join().unwrap();
}