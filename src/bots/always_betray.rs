use rand::Rng;

use super::{Bot, Dilemma, Turn};

#[derive(Debug, Default)]
pub struct AlwaysBetray {}

impl Bot for AlwaysBetray {
    fn new() -> Self {
        AlwaysBetray {}
    }

    /// This strategy will always betray the other player
    fn turn(&self, _: &Vec<Turn>) -> Dilemma {
        Dilemma::Betray
    }
}
