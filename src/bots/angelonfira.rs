use super::{Bot, Dilemma};

#[derive(Debug, Default)]
pub struct AngelOnFira {}

impl Bot for AngelOnFira {
    fn new() -> Self {
        AngelOnFira {}
    }
    
    fn turn(&self) -> Dilemma {
        Dilemma::Silence
    }
}
