use rayon::iter::{IntoParallelIterator, ParallelIterator};
use std::time::Instant;

fn parallel_monte_carlo_pi(points: usize) -> f64 {
    let within_circle = (0..points)
        .into_par_iter()
        .filter_map(|_| {
            let x = rand::random::<f64>() * 2f64 - 1f64;
            let y = rand::random::<f64>() * 2f64 - 1f64;
            if x * x + y * y <= 1f64 {
                Some(1)
            } else {
                None
            }
        })
        .count();
    4f64 * within_circle as f64 / points as f64
}

fn sequential_monte_carlo_pi(points: usize) -> f64 {
    let within_circle = (0..points)
        .filter_map(|_| {
            let x = rand::random::<f64>() * 2f64 - 1f64;
            let y = rand::random::<f64>() * 2f64 - 1f64;
            if x * x + y * y <= 1f64 {
                Some(1)
            } else {
                None
            }
        })
        .count();
    4f64 * within_circle as f64 / points as f64
}

struct EvenNumbers {
    value: i32,
}

impl EvenNumbers {
    fn new() -> Self {
        Self { value: 2 }
    }
}

impl Iterator for EvenNumbers {
    type Item = i32;

    fn next(&mut self) -> Option<Self::Item> {
        let old_value = Some(self.value);
        self.value += 2;
        old_value
    }
}

fn main() {
    let result: Vec<i32> = EvenNumbers::new()
        .skip(10)
        .map(|x| 3 * x)
        .take(10)
        .collect();
    println!("{:#?}", result);

    let sum = (1..100).fold(0, |acc, x| acc + 2 * x);
    println!("{}", sum);

    let my_result: i32 = itertools::izip!(EvenNumbers::new(), 1..)
        .map(|x| x.0 + x.1)
        .take(10)
        .sum();

    println!("{}", my_result);

    let nums = (1..100).collect::<Vec<i32>>();
    println!("{}", nums[0]);

    let parallel_result: i32 = (1..5000)
        .into_par_iter()
        .map(|x| 2 * x + 1)
        .filter(|x| *x > 100)
        .sum();

    println!("parallel computation: {}", parallel_result);

    for num_points in [10, 100, 1000, 10000, 100000, 1000000, 1000000, 10000000] {
        let start = Instant::now();
        let pi_res = parallel_monte_carlo_pi(num_points);
        let duration = start.elapsed();
        println!(
            "pi = {} from {} points in parallel, took {:?}",
            pi_res, num_points, duration
        );
        let start_seq = Instant::now();
        let pi_res_seq = sequential_monte_carlo_pi(num_points);
        let duration_seq = start_seq.elapsed();
        println!(
            "pi = {} from {} points in sequential, took {:?}",
            pi_res_seq, num_points, duration_seq
        );
        let faster = duration_seq.as_secs_f64() / duration.as_secs_f64();
        println!("Improvement for parallel: {:?}", faster);
    }
}
