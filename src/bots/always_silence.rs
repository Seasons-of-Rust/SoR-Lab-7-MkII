use rand::Rng;

use super::{Bot, Dilemma, Turn};

#[derive(Debug, Default)]
pub struct AlwaysSilence {}

impl Bot for AlwaysSilence {
    fn new() -> Self {
        AlwaysSilence {}
    }

    /// This strategy will always stay silent
    fn turn(&self, _: &Vec<Turn>) -> Dilemma {
        Dilemma::Silence
    }
}
