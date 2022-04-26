use rayon::prelude::*;
use std::sync::mpsc;
use std::thread;
use std::time::Duration;

use rand::Rng;

fn sum_of_squares(input: &[i64]) -> i64 {
    input.par_iter().map(|&i| 2 * i).sum()
}

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

    // mpsc is an acronym for multiple producer, single consumer.
    let (tx, rx) = mpsc::channel();

    let tx1 = tx.clone();
    thread::spawn(move || {
        let vals = vec![
            String::from("hi"),
            String::from("from"),
            String::from("the"),
            String::from("thread"),
        ];

        let mut rng = rand::thread_rng();

        for val in vals {
            let rand_millis = rng.gen_range(1..1000);
            thread::sleep(Duration::from_millis(rand_millis));
            tx1.send(val).unwrap();
        }
    });

    thread::spawn(move || {
        let vals = vec![
            String::from("more"),
            String::from("messages"),
            String::from("for"),
            String::from("you"),
        ];

        let mut rng = rand::thread_rng();

        for val in vals {
            let rand_millis = rng.gen_range(1..100);
            thread::sleep(Duration::from_millis(rand_millis));
            tx.send(val).unwrap();
        }
    });

    for received in rx {
        println!("Got: {}", received);
    }

    let v: Vec<i64> = (0..50000).collect();
    let res = sum_of_squares(&v);
    println!("Sum is {}", res);
}
