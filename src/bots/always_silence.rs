use rand::Rng;

use super::{Bot, Dilemma};

#[derive(Debug, Default)]
pub struct AlwaysSilence {}

impl Bot for AlwaysSilence {
    fn new() -> Self {
        AlwaysSilence
     {}
    }

    fn turn(&self) -> Dilemma {
        Dilemma::Silence
    }
}
