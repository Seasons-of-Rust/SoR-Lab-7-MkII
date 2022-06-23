mod bots;

use bots::Simulation;

fn main() {
    let sim = Simulation {};

    let results = sim.run();

    // Print the results
    for bot in results.keys() {
        println!("{:?}: {}", bot, results[bot]);
    }
}
