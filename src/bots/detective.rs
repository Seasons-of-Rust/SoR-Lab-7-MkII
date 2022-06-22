use rand::Rng;

use super::{Bot, Dilemma, Turn};

#[derive(Debug, Default)]
pub struct Detective {
    should_i_exploit: bool,
}

impl Bot for Detective {
    fn new() -> Self {
        Detective {
            should_i_exploit: false,
        }
    }

    /// From https://github.com/carykh/PrisonersDilemmaTournament/blob/main/code/exampleStrats/detective.py
    ///
    /// Strategy described in Nicky Case's "The Evolution of Trust"
    /// https://ncase.me/trust/
    ///
    /// DETECTIVE: First: I analyze you. I start:
    /// Cooperate, Cheat, Cooperate, Cooperate.
    /// If you cheat back, I'll act like [Tit for Tat].
    /// If you never cheta back, I'll act like [alwaysDefect],
    /// to exploit you. Elementary, my dear Watson.
    fn turn(&mut self, history: &Vec<Turn>) -> Dilemma {
        let testing_schedule = vec![
            Dilemma::Silence,
            Dilemma::Betray,
            Dilemma::Silence,
            Dilemma::Silence,
        ];

        let game_length = history.len();

        let mut choice = None;

        // We're still in that initial testing stage
        if game_length < 4 {
            choice = Some(testing_schedule[game_length]);
        // Time to analyze the testing stage and decide what to do based on what the opponent did in that time!
        } else if game_length == 4 {
            if history
                .iter()
                .filter(|&x| matches!(x.other_bot, Dilemma::Betray))
                .count()
                == 0
            {
                self.should_i_exploit = true;
            } else {
                self.should_i_exploit = false;
            }
        }

        // Now we're in the game
        if game_length >= 4 {
            if self.should_i_exploit {
                choice = Some(Dilemma::Betray);
            } else {
                choice = history.last().map(|x| x.other_bot);
            }
        }

        choice.unwrap_or(Dilemma::Silence)
    }
}
