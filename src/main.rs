extern crate rand;
extern crate rcmd;

use rand::{ OsRng, Rng };
use rcmd::RollCmd;
use std::error::Error;

fn main() {
    // Attempt to retrieve randomness from OsRng
    let mut rng = match OsRng::new() {
        Ok(rng) => rng,
        Err(e)  => {
            println!("{}", e.description());
            return;
        }
    };

    //
    let rolls: Vec<_> = std::env::args()
        .filter_map(|arg| arg.parse::<RollCmd>().ok())
        .map(|cmd| cmd.result(|max| rng.gen_range(0, max) + 1))
        .collect();

    for roll in rolls { println!("{}", roll); }
}

/*
// Shouldn't this be a datatype with a Rollable typeclass?
// Not simpler in the small, but more flexible in the large
fn roll_d(sides: u32) -> u32 { // Don't see many use cases for u64 dice, but change if you need it :shrug:
}
*/
