use std::collections::HashMap;
use strum::IntoEnumIterator;
use strum_macros::EnumIter;

use self::{
    always_betray::AlwaysBetray, always_silence::AlwaysSilence, angelonfira::AngelOnFira,
    detective::Detective, fifty_fifty::FiftyFifty, grim_trigger::GrimTrigger,
};

mod always_betray;
mod always_silence;
mod angelonfira;
mod detective;
mod fifty_fifty;
mod grim_trigger;

trait Bot {
    fn new() -> Self
    where
        Self: Sized;

    fn turn(&mut self, history: &[Turn]) -> Dilemma;
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

#[derive(Debug, EnumIter, Eq, Hash, PartialEq, Clone, Copy)]
pub enum Bots {
    AngelOnFira,
    FiftyFifty,
    AlwaysSilence,
    AlwaysBetray,
    GrimTrigger,
    Detective,
}

impl Bots {
    fn objects(&self) -> Box<dyn Bot> {
        match self {
            Bots::AngelOnFira => Box::new(AngelOnFira::new()),
            Bots::FiftyFifty => Box::new(FiftyFifty::new()),
            Bots::AlwaysSilence => Box::new(AlwaysSilence::new()),
            Bots::AlwaysBetray => Box::new(AlwaysBetray::new()),
            Bots::GrimTrigger => Box::new(GrimTrigger::new()),
            Bots::Detective => Box::new(Detective::new()),
        }
    }
}

pub struct Simulation {}

impl Simulation {
    pub fn run(&self) -> HashMap<Bots, i32> {
        let mut results: HashMap<Bots, i32> = HashMap::new();

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

                // Simulate 1000 fights between bot1 and bot2
                for _ in 0..1_000 {
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
