/// montecarlo approximation of pi in rust
extern crate num_cpus;
extern crate rand;

use rand::{thread_rng, Rng};
use std::thread;

struct RFloat {
    x: f32,
    y: f32,
}

#[inline]
fn two_r() -> RFloat {
    let mut rng = thread_rng();
    let x = rng.gen::<f32>();
    let y = rng.gen::<f32>();
    RFloat { x, y }
}

fn in_circle(j: &RFloat) -> u64 {
    let f = (j.x * j.x + j.y * j.y).sqrt();
    if f <= 1.0 {
        1
    } else {
        0
    }
}

pub fn multi_calc_pi(samples: u64, cores: Option<u64>) -> f64 {
    let threads = thread_count(cores);
    let mut children = vec![];
    let iterations: u64 = samples / threads;

    for _ in 0..threads {
        children.push(thread::spawn(move || -> u64 {
            (0..iterations).fold(0, |hits, _| hits + in_circle(&two_r()))
        }));
    }
    let mut intermediate_sums = vec![];
    for child in children {
        // collect each child thread's return-value
        let intermediate_sum = child.join().unwrap();
        intermediate_sums.push(intermediate_sum);
    }
    let final_result = intermediate_sums.iter().sum::<u64>();
    //return pi
    4.0 * final_result as f64 / samples as f64
}

pub fn single_calc_pi(samples: u64) -> f64 {
    let result = (0..samples).fold(0, |hits, _| hits + in_circle(&two_r()));
    4.0 * result as f64 / samples as f64
}

fn thread_count(cores: Option<u64>) -> u64 {
    let mut threads: u64 = match cores {
        Some(count) => count,
        None => num_cpus::get_physical() as u64,
    };
    if threads > num_cpus::get() as u64 {
        threads = num_cpus::get() as u64
    };
    threads
}