use rust_concurrent::exchanger::Exchanger;
use std::sync::Arc;
use std::thread;

fn main() {
    println!("Start Exchange!");

    let exchanger:Arc<Exchanger<i32>> = Arc::new(Exchanger::new());
    let exchanger1:Arc<Exchanger<i32>> = Arc::clone(&exchanger);
    let exchanger2:Arc<Exchanger<i32>> = Arc::clone(&exchanger);
    let exchanger3:Arc<Exchanger<i32>> = Arc::clone(&exchanger);
    let exchanger4:Arc<Exchanger<i32>> = Arc::clone(&exchanger);

    let t1 = thread::spawn(move || {
        let val = 1;
        let res = exchanger1.exchange(val);
        println!("Thread1: {}", res);
    });

    let t2 = thread::spawn(move || {
        let val = 2;
        let res = exchanger2.exchange(val);
        println!("Thread2: {}", res);
    });

    let t3 = thread::spawn(move || {
        let val = 3;
        let res = exchanger3.exchange(val);
        println!("Thread3: {}", res);
    });

    let t4 = thread::spawn(move || {
        let val = 4;
        let res = exchanger4.exchange(val);
        println!("Thread4: {}", res);
    });

    t1.join().unwrap();
    t2.join().unwrap();
    t3.join().unwrap();
    t4.join().unwrap();

    println!("End Exchange!");
}