extern crate montepi;

use montepi::calc_pi;

fn main(){
    let samples = 100_000_000;
    println!("π ~ {}",calc_pi(samples));
}