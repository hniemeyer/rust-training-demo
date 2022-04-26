use std::thread;
use std::time::Duration;

use rand::Rng;

fn main() {
    let mut my_threads = vec![];

    for i in 1..10 {
        my_threads.push(thread::spawn(move || {
            let mut rng = rand::thread_rng();
            let rand_millis = rng.gen_range(1..100);
            println!("hi from thread number {}!", i);
            thread::sleep(Duration::from_millis(rand_millis));
            println!("Thread number {} slept for {} millis!", i, rand_millis);
        }));
    }

    println!("Now we wait for the threads.");
    for child_thread in my_threads {
        // Wait for the thread to finish. Returns a result.
        let _ = child_thread.join();
    }
}
