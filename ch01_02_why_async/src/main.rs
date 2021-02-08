use std::thread;
use std::time::{
    Duration,
    Instant
};

use futures::join;
use tokio::time;

fn sync_get() {
    thread::sleep(Duration::from_millis(100));
    println!("sync_get finished");
}
async fn async_get() {
    time::sleep(time::Duration::from_millis(100)).await;
    println!("async_get finished");
}

fn threaded_fetch() {
    // spawn both threads
    let thread_one = thread::spawn(|| sync_get());
    let thread_two = thread::spawn(|| sync_get());
    let thread_three = thread::spawn(|| sync_get());
    let thread_four = thread::spawn(|| sync_get());
    let thread_five = thread::spawn(|| sync_get());

    // join threads waiting for them to finish
    thread_one.join().unwrap();
    thread_two.join().unwrap();
    thread_three.join().unwrap();
    thread_four.join().unwrap();
    thread_five.join().unwrap();
}

async fn async_fetch() -> ((), (), (), (), ()) {
    // spawn both threads
    let future_one = async_get();
    let future_two = async_get();
    let future_three = async_get();
    let future_four = async_get();
    let future_five = async_get();

    // join futures waiting for them to finish
    join!(future_one, future_two, future_three, future_four, future_five)
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    {
        let started = Instant::now();
        threaded_fetch();
        let duration = started.elapsed();
        println!("-----------------");
        println!("total: {:?}", duration)
    }
    println!();
    {
        let started = Instant::now();
        async_fetch().await;
        let duration = started.elapsed();
        println!("------------------");
        println!("total: {:?}", duration)
    }
    Ok(())
}