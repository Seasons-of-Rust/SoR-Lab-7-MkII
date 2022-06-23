use std::collections::HashMap;

use log::debug;
use rand::Rng;
use strum::IntoEnumIterator;

use crate::bots::{Bots, Dilemma, Turn};

pub struct Simulation {}

impl Simulation {
    /// Run the simulation of all bots fighting against each other. Each bot
    /// will
    pub fn run(&self) -> HashMap<Bots, f32> {
        let mut results: HashMap<Bots, f32> = HashMap::new();
        let mut rng = rand::thread_rng();

        // Simulate each bot fighting against each other
        for bot_1_type in Bots::iter() {
            let mut new_bot_1 = bot_1_type.objects();
            for bot_2_type in Bots::iter() {
                // Prevent bots from fighting against themselves
                if bot_1_type == bot_2_type {
                    continue;
                }

                let mut new_bot_2 = bot_2_type.objects();

                let mut bot_1_history = Vec::new();
                let mut bot_2_history = Vec::new();

                // Simulate a number of fights between bot1 and bot2

                // Pick a random number of fights to simulate
                // https://github.com/carykh/PrisonersDilemmaTournament/blob/97a0f1b366a61e81ec2250d18c2055d4d90b40a3/code/prisonersDilemma.py#L45
                let num_of_fights = 1000 + rng.gen_range(0..100);

                for _ in 0..num_of_fights {
                    let bot1_dilemma = new_bot_1.turn(&bot_1_history);
                    let bot2_dilemma = new_bot_2.turn(&bot_2_history);

                    // Add the histories
                    bot_1_history.push(Turn {
                        you: bot1_dilemma,
                        other_bot: bot2_dilemma,
                    });

                    bot_2_history.push(Turn {
                        you: bot2_dilemma,
                        other_bot: bot1_dilemma,
                    });
                }

                // Calculate the scores of both bots
                let (bot_1_score, bot_2_score) =
                    bot_1_history
                        .iter()
                        .fold((0, 0), |(bot_1_score, bot_2_score), turn| {
                            match (turn.you, turn.other_bot) {
                                (Dilemma::Silence, Dilemma::Silence) => {
                                    (bot_1_score + 3, bot_2_score + 3)
                                }
                                (Dilemma::Silence, Dilemma::Betray) => {
                                    (bot_1_score, bot_2_score + 5)
                                }
                                (Dilemma::Betray, Dilemma::Silence) => {
                                    (bot_1_score + 5, bot_2_score)
                                }
                                (Dilemma::Betray, Dilemma::Betray) => {
                                    (bot_1_score + 1, bot_2_score + 1)
                                }
                            }
                        });

                let bot_1_score = bot_1_score as f32 / bot_1_history.len() as f32;
                let bot_2_score = bot_2_score as f32 / bot_2_history.len() as f32;

                debug!(
                    "{:?} ({}) vs {:?} ({}) over {} fights",
                    bot_1_type, bot_1_score, bot_2_type, bot_2_score, num_of_fights
                );

                // Add the score to the results
                results
                    .entry(bot_1_type)
                    .and_modify(|e| *e += bot_1_score)
                    .or_insert(bot_1_score);

                results
                    .entry(bot_2_type)
                    .and_modify(|e| *e += bot_2_score)
                    .or_insert(bot_2_score);
            }
        }

        // Return the results
        results
    }
}
