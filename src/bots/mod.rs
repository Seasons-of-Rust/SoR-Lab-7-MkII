use std::collections::HashMap;
use strum::IntoEnumIterator;
use strum_macros::EnumIter;

use self::{
    always_betray::AlwaysBetray, always_silence::AlwaysSilence, angelonfira::AngelOnFira,
    fifty_fifty::FiftyFifty, grim_trigger::GrimTrigger,
};

mod always_betray;
mod always_silence;
mod angelonfira;
mod fifty_fifty;
mod grim_trigger;

trait Bot {
    fn new() -> Self
    where
        Self: Sized;

    fn turn(&self, history: &Vec<Turn>) -> Dilemma;
}

struct Turn {
    you: Dilemma,
    other_bot: Dilemma,
}

#[derive(Copy, Clone)]
pub enum Dilemma {
    Silence,
    Betray,
}

#[derive(Debug, EnumIter, Eq, Hash, PartialEq, Clone)]
pub enum Bots {
    AngelOnFira,
    FiftyFifty,
    AlwaysSilence,
    AlwaysBetray,
    GrimTrigger,
}

impl Bots {
    fn objects(&self) -> Box<dyn Bot> {
        match self {
            Bots::AngelOnFira => Box::new(AngelOnFira::new()),
            Bots::FiftyFifty => Box::new(FiftyFifty::new()),
            Bots::AlwaysSilence => Box::new(AlwaysSilence::new()),
            Bots::AlwaysBetray => Box::new(AlwaysBetray::new()),
            Bots::GrimTrigger => Box::new(GrimTrigger::new()),
        }
    }
}

pub struct Simulation {}

impl Simulation {
    pub fn run(&self) -> HashMap<Bots, i32> {
        let mut rng = rand::thread_rng();
        let mut results: HashMap<Bots, i32> = HashMap::new();

        // Simulate each bot fighting against each other bot 1 million times
        for _ in 0..1_000 {
            for (i, bot1) in Bots::iter().enumerate() {
                let bot_1 = bot1.objects();
                for bot2 in Bots::iter().skip(i + 1) {
                    let bot_2 = bot2.objects();

                    let mut bot1_score = 0;
                    let mut bot2_score = 0;

                    let mut bot_1_history = Vec::new();
                    let mut bot_2_history = Vec::new();

                    // Simulate 1000 fights between bot1 and bot2
                    for _ in 0..1_000 {
                        let bot1_dilemma = bot_1.turn(&bot_1_history);
                        let bot2_dilemma = bot_2.turn(&bot_2_history);

                        // Add the histories
                        bot_1_history.push(Turn {
                            you: bot1_dilemma,
                            other_bot: bot2_dilemma,
                        });

                        bot_2_history.push(Turn {
                            you: bot2_dilemma,
                            other_bot: bot1_dilemma,
                        });

                        // Determine the outcome of the fight
                        match (bot1_dilemma, bot2_dilemma) {
                            (Dilemma::Silence, Dilemma::Silence) => {
                                bot1_score += 3;
                                bot2_score += 3;
                            }
                            (Dilemma::Silence, Dilemma::Betray) => {
                                bot2_score += 5;
                            }
                            (Dilemma::Betray, Dilemma::Silence) => {
                                bot1_score += 5;
                            }
                            (Dilemma::Betray, Dilemma::Betray) => {
                                bot1_score += 1;
                                bot2_score += 1;
                            }
                        }
                    }

                    // Add the score to the results
                    results
                        .entry(bot1.clone())
                        .and_modify(|e| *e += bot1_score)
                        .or_insert(bot1_score);

                    results
                        .entry(bot2)
                        .and_modify(|e| *e += bot2_score)
                        .or_insert(bot2_score);
                }
            }
        }

        // Return the results
        results
    }
}
