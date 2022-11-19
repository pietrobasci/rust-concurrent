use std::thread;
use std::thread::sleep;
use std::time::Duration;
use rust_concurrent::f_cache::FCache;
use std::sync::Arc;


fn main() {
    let f_cache = Arc::new(FCache::new());
    let f_cache1 = Arc::clone(&f_cache.clone());
    let f_cache2 = Arc::clone(&f_cache.clone());


    let t1 = thread::spawn(move || {
        println!("Thread1 start!");
        let k = 1;
        let res = f_cache.get(k, |&_k|{
            sleep(Duration::from_secs(10));
            1 });
        println!("Thread1 done! (res:{})", *res);
    });

    sleep(Duration::from_secs(5));

    let t2 = thread::spawn(move || {
        println!("Thread2 start!");
        let k = 2;
        let res = f_cache1.get(k, |&_k|{
            sleep(Duration::from_secs(10));
            2 });
        println!("Thread2 done! (res:{})", res);
    });

    sleep(Duration::from_secs(5));

    let t3 = thread::spawn(move || {
        println!("Thread3 start!");
        let k = 1;
        let res = f_cache2.get(k, |&_k|{
            sleep(Duration::from_secs(20));
            1 });
        println!("Thread3 done! (res:{})", res);
    });

    println!("Done");

    t1.join().unwrap();
    t2.join().unwrap();
    t3.join().unwrap();

}