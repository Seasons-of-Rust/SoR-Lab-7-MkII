use super::{Bot, Dilemma, Turn};

#[derive(Debug, Default)]
pub struct AlwaysSilence {}

impl Bot for AlwaysSilence {
    fn new() -> Self {
        AlwaysSilence {}
    }

    /// This strategy will always stay silent
    fn turn(&mut self, _: &[Turn]) -> Dilemma {
        Dilemma::Silence
    }
}
