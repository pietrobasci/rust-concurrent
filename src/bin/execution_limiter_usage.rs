use rust_concurrent::execution_limiter::ExecutionLimiter;
use std::sync::Arc;
use std::thread;
use std::thread::sleep;
use std::time::Duration;


fn task_1() -> i32 {
    println!("Task1 start!");
    sleep(Duration::from_secs(10));
    1
}
fn task_2() -> i32 {
    println!("Task2 start!");
    sleep(Duration::from_secs(15));
    2
}
fn task_3() -> i32 {
    println!("Task3 start!");
    sleep(Duration::from_secs(1));
    3
}

fn main() {
    let cdl = Arc::new(ExecutionLimiter::new(2));
    let cdl1 = Arc::clone(&cdl);
    let cdl2 = Arc::clone(&cdl);
    let cdl3 = Arc::clone(&cdl);

    let t1 = thread::spawn(move || {
        println!("Thread1 start!");
        sleep(Duration::from_secs(2));
        cdl1.execute(task_1);
        println!("Thread1 done!");
    });

    let t2 = thread::spawn(move || {
        println!("Thread2 start!");
        cdl2.execute(task_2);
        println!("Thread2 done!");
    });

    let t3 = thread::spawn(move || {
        println!("Thread3 start!");
        cdl3.execute(task_3);
        println!("Thread3 done!");
    });

    println!("Done");

    t1.join().unwrap();
    t2.join().unwrap();
    t3.join().unwrap();
}