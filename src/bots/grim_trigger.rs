use super::{Bot, Dilemma, Turn};

#[derive(Debug, Default)]
pub struct GrimTrigger {}

impl Bot for GrimTrigger {
    fn new() -> Self {
        GrimTrigger {}
    }

    /// From https://github.com/carykh/PrisonersDilemmaTournament/blob/main/code/exampleStrats/grimTrigger.py
    ///
    /// Strategy known as "Grim Trigger" or "Grudger".
    /// We will cooperate repeatedly until our opponent betrays us once.
    /// Then, we will get angry and defect for the rest of time.
    ///
    /// In this implementation, I used the memory variable to store Grim Trigger's state of mind.
    /// memory is true if Grim Trigger has been wronged, and false if it hasn't.
    fn turn(&mut self, history: &[Turn]) -> Dilemma {
        match history
            .iter()
            // If the other bot every betrayed, always betray
            .any(|turn| matches!(turn.other_bot, Dilemma::Betray))
        {
            true => Dilemma::Betray,
            false => Dilemma::Silence,
        }
    }
}
