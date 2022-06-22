use super::{Bot, Dilemma, Turn};

#[derive(Debug, Default)]
pub struct AngelOnFira {}

impl Bot for AngelOnFira {
    fn new() -> Self {
        AngelOnFira {}
    }

    /// This strategy is very nice. It will never betray you!
    fn turn(&self, _: &Vec<Turn>) -> Dilemma {
        Dilemma::Silence
    }
}
