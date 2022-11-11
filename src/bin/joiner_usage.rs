use rust_concurrent::joiner::Joiner;
use std::sync::Arc;
use std::thread;
use std::thread::sleep;
use std::time::Duration;

fn main() {
    let joiner = Arc::new(Joiner::new(3));
    let joiner1 = Arc::clone(&joiner);
    let joiner2 = Arc::clone(&joiner);
    let joiner3 = Arc::clone(&joiner);
    let joiner4 = Arc::clone(&joiner);
    let joiner5 = Arc::clone(&joiner);

    let t1 = thread::spawn(move || {
        sleep(Duration::from_secs(2));
        let res = joiner.supply(1.4);
        println!("Thread1 done! Got:{:?}", res);
        res
    });

    let t2 = thread::spawn(move || {
        sleep(Duration::from_secs(3));
        let res = joiner1.supply(2.5);
        println!("Thread2 done! Got:{:?}", res);
        res
    });

    let t3 = thread::spawn(move || {
        sleep(Duration::from_secs(5));
        let res = joiner2.supply(3.5);
        println!("Thread3 done! Got:{:?}", res);
        res
    });

    let t4 = thread::spawn(move || {
        sleep(Duration::from_secs(1));
        let res = joiner3.supply(4.5);
        println!("Thread4 done! Got:{:?}", res);
        res
    });

    let t5 = thread::spawn(move || {
        sleep(Duration::from_secs(7));
        let res = joiner4.supply(5.5);
        println!("Thread5 done! Got:{:?}", res);
        res
    });

    let t6 = thread::spawn(move || {
        sleep(Duration::from_secs(8));
        let res = joiner5.supply(6.5);
        println!("Thread6 done! Got:{:?}", res);
        res
    });

    println!("Done");

    let v1 = t1.join().unwrap();
    let v2 = t2.join().unwrap();
    let v3 = t3.join().unwrap();
    let v4 = t4.join().unwrap();
    let v5 = t5.join().unwrap();
    let v6 = t6.join().unwrap();

    println!("------------------------------------");
    println!("{:?}", v1);
    println!("{:?}", v2);
    println!("{:?}", v3);
    println!("{:?}", v4);
    println!("{:?}", v5);
    println!("{:?}", v6);

}