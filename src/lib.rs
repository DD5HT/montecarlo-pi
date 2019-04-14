/// montecarlo approximation of pi in rust
use rand::{thread_rng, Rng};
use std::thread;

struct RFloats {
    x: f64,
    y: f64,
}

#[inline]
fn two_r() -> RFloats {
    let mut rng = thread_rng();
    let x = rng.gen::<f64>();
    let y = rng.gen::<f64>();
    RFloats { x, y }
}

fn in_circle(j: &RFloats) -> u64 {
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

///A single function to calculate
pub fn single_calc_pi(samples: u64) -> f64 {
    4.0 * (0..samples).fold(0, |hits, _| hits + in_circle(&two_r())) as f64 / samples as f64
}

/// Return the possible thread counts, uses number of physical cores as a default.
fn thread_count(cores: Option<u64>) -> u64 {
    let total_threads = num_cpus::get() as u64;
    let threads: u64 = match cores {
        Some(count) => count,
        None => num_cpus::get_physical() as u64,
    };

    if threads > total_threads {
        total_threads
    } else {
        threads
    }
}
