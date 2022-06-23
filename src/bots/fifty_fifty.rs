use rand::Rng;

use super::{Bot, Dilemma, Turn};

#[derive(Debug, Default)]
pub struct FiftyFifty {}

impl Bot for FiftyFifty {
    fn new() -> Self {
        FiftyFifty {}
    }

    fn turn(&mut self, _: &[Turn]) -> Dilemma {
        // Randomly choose between silence and betrayal
        let mut rng = rand::thread_rng();
        match rng.gen_range(0..2) {
            // If the number is 0, return Silence
            0 => Dilemma::Silence,
            // If the number is 1, return Betray
            1 => Dilemma::Betray,
            _ => unreachable!(),
        }
    }
}
