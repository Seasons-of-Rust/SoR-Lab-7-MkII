use rand::Rng;

use super::{Bot, Dilemma};

#[derive(Debug, Default)]
pub struct AlwaysBetray {}

impl Bot for AlwaysBetray {
    fn new() -> Self {
        AlwaysBetray {}
    }
    
    fn turn(&self) -> Dilemma {
        Dilemma::Betray
    }
}
