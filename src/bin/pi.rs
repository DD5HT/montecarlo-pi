extern crate montepi;
extern crate structopt;

use structopt::StructOpt;

use montepi::*;

#[derive(StructOpt, Debug)]
#[structopt(name = "pi", about = "Calculates π using a Monte Carlo algorithm.")]
struct Opt {
    ///specify the amount of samples
    #[structopt(short = "s", long = "samples")]
    samples: Option<u64>,
    ///use -m for multicore variant using the amount of physical cores
    #[structopt(short = "m", long = "multi")]
    multicore: bool,
    ///specify the amount of threads
    #[structopt(short = "t", long = "threads")]
    threads: Option<u64>,
}

fn main() {
    let opt = Opt::from_args();
    let default = 100_000_000;
    let output: f64;
    let threads = opt.threads;
    if opt.multicore || threads.is_some() {
        println!("Running Multicore variant!");
        if let Some(samples) = opt.samples {
            output = multi_calc_pi(samples, threads);
        } else {
            output = multi_calc_pi(default, threads);
        }
    } else {
        println!("Running Singlecore variant!");
        if let Some(samples) = opt.samples {
            output = single_calc_pi(samples);
        } else {
            output = single_calc_pi(default);
        }
    }
    if opt.samples.is_none() {
        println!("Using default value of: {}", default)
    }
    println!("π ~ {}", output);
}
