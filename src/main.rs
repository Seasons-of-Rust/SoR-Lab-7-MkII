use itertools::Itertools;
use log::info;
use std::io::Write;

use crate::simulation::Simulation;

mod bots;
mod simulation;

fn main() {
    // Set up logging
    pretty_env_logger::init();

    // Set up the simulation
    let sim = Simulation {};

    // Run the simulation
    let results = sim.run();

    // Print the results
    info!("The winner is...");
    for (bot, score) in results
        .keys()
        .map(|b| (b, results[b]))
        .sorted_by(|a, b| b.1.partial_cmp(&a.1).unwrap())
    {
        info!("{:?}: {}", bot, score);
    }
}
