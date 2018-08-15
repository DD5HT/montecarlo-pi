// montecarlo approximation of pi in rust
extern crate rand;
extern crate num_cpus;

use std::thread;
use rand::{Rng, thread_rng};

pub struct RFloat {
    x: f64,
    y: f64,
}

#[inline]
pub fn two_r() -> RFloat {
    let mut rng  = thread_rng();
    let x = rng.gen::<f64>();
    let y = rng.gen::<f64>();
    RFloat{x: x, y: y}
}

fn in_circle(j: RFloat) -> u64 {
    let f = (j.x*j.x + j.y*j.y).sqrt();
    if f <= 1.0 {1} else{0}
}

fn calc_pi(samples: u64) -> f64 {
    let threads: u64 = num_cpus::get_physical() as u64;
    let mut children = vec![];
    let iterations: u64 = samples / threads;
    println!("Using {} physical cores:", threads);

    for _ in 0..threads {
        children.push(thread::spawn(move || -> u64 {
            (0..iterations).fold(0,|hits, _| hits + in_circle(two_r()))   
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
    4.0 * final_result as f64 / (iterations * threads) as f64
}

fn main(){
    let samples = 1_000_000;
    println!("π ~ {}",calc_pi(samples));
}