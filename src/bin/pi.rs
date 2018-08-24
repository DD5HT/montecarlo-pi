extern crate montepi;
extern crate structopt;

use structopt::StructOpt;

use montepi::*;

#[derive(StructOpt, Debug)]
#[structopt(
    name = "pi",
    about = "Calculates π using a Monte Carlo algorithm."
)]
struct Opt {
    #[structopt(short = "s", long = "samples")]
    samples: Option<u64>,
    #[structopt(short = "m", long = "multi")]
    multicore: bool,
}

fn main() {
    let opt = Opt::from_args();
    let default = 100_000_000;
    let output;
    if opt.multicore {
        println!("Running Multicore variant!");
        if let Some(samples) = opt.samples {
            output = multi_calc_pi(samples);
        } else {
            output = multi_calc_pi(default);
        }
    } else {
        println!("Running Singlecore variant!");
        if let Some(samples) = opt.samples {
            output = single_calc_pi(samples);
        } else {
            output = single_calc_pi(default);
        }
    }
    if !opt.samples.is_some() {
        println!("Using default value of: {}", default)
    }
    println!("π ~ {}", output);
}
