/// montecarlo approximation of pi in rust
extern crate num_cpus;
extern crate rand;

use rand::{thread_rng, Rng};
use std::thread;

struct RFloat {
    x: f64,
    y: f64,
}

#[inline]
fn two_r() -> RFloat {
    let mut rng = thread_rng();
    let x = rng.gen::<f64>();
    let y = rng.gen::<f64>();
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

pub fn multi_calc_pi(samples: u64) -> f64 {
    let threads: u64 = num_cpus::get_physical() as u64;
    println!("Using {} physical cores:", threads);

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
