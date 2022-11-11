use rust_concurrent::count_down_latch::CountDownLatch;
use std::sync::Arc;
use std::thread;
use std::thread::sleep;
use std::time::Duration;

fn main() {
    let cdl = Arc::new(CountDownLatch::new(2));
    let cdl1 = Arc::clone(&cdl);
    let cdl2 = Arc::clone(&cdl);

    let t1 = thread::spawn(move || {
        sleep(Duration::from_secs(2));
        cdl1.count_down();
        sleep(Duration::from_secs(5));
        println!("Thread1 done!");
    });

    let t2 = thread::spawn(move || {
        sleep(Duration::from_secs(3));
        cdl2.count_down();
        sleep(Duration::from_secs(6));
        println!("Thread2 done!");
    });

    cdl.wait();
    println!("Done");

    t1.join().unwrap();
    t2.join().unwrap();
}