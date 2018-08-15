#[macro_use]
extern crate structopt;
extern crate montepi;

use structopt::StructOpt;

use montepi::calc_pi;

#[derive(StructOpt, Debug)]
#[structopt(name = "pi", about = "Calculates π using a Monte Carlo algorithm.")]
struct Opt {

    #[structopt(short = "s", long = "samples")]
    samples: Option<u64>,
}

fn main(){
    let opt = Opt::from_args();
    if let Some(samples) = opt.samples {
        println!("π ~ {}",calc_pi(samples));
    }else {
        let default = 10000;
        println!("Using default value of {}:", default);
        println!("π ~ {}",calc_pi(default));
    }
}