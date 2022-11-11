use rust_concurrent::looper::Looper;
use std::thread;
use std::thread::sleep;
use std::time::Duration;


fn process(msg: String){
    sleep(Duration::from_secs(1));
    println!("{}", msg);
}
fn cleanup(){
    println!("Cleaned!");
}

fn main() {
    let looper = Looper::new(process, cleanup);
    let looper1 = looper.clone();
    let looper2 = looper.clone();


    let t1 = thread::spawn(move || {
        looper.send("Hello ".to_string());
        println!("Thread1 done!");
    });

    let t2 = thread::spawn(move || {
        looper1.send("World!".to_string());
        println!("Thread2 done!");
    });

    let t3 = thread::spawn(move || {
        looper2.send("!!!!".to_string());
        println!("Thread3 done!");
    });

    sleep(Duration::from_secs(5));
    println!("Done");

    t1.join().unwrap();
    t2.join().unwrap();
    t3.join().unwrap();

}