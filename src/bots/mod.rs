use strum_macros::EnumIter;

use self::{
    always_betray::AlwaysBetray, always_silence::AlwaysSilence, angelonfira::AngelOnFira,
    detective::Detective, fifty_fifty::FiftyFifty, grim_trigger::GrimTrigger,
};

mod always_betray;
mod always_silence;
mod angelonfira;
mod detective;
mod fifty_fifty;
mod grim_trigger;

pub trait Bot {
    fn new() -> Self
    where
        Self: Sized;

    fn turn(&mut self, history: &[Turn]) -> Dilemma;
}

pub struct Turn {
    pub you: Dilemma,
    pub other_bot: Dilemma,
}

#[derive(Copy, Clone)]
pub enum Dilemma {
    Silence,
    Betray,
}

#[derive(Debug, EnumIter, Eq, Hash, PartialEq, Clone, Copy)]
pub enum Bots {
    AngelOnFira,
    FiftyFifty,
    AlwaysSilence,
    AlwaysBetray,
    GrimTrigger,
    Detective,
}

impl Bots {
    pub fn objects(&self) -> Box<dyn Bot> {
        match self {
            Bots::AngelOnFira => Box::new(AngelOnFira::new()),
            Bots::FiftyFifty => Box::new(FiftyFifty::new()),
            Bots::AlwaysSilence => Box::new(AlwaysSilence::new()),
            Bots::AlwaysBetray => Box::new(AlwaysBetray::new()),
            Bots::GrimTrigger => Box::new(GrimTrigger::new()),
            Bots::Detective => Box::new(Detective::new()),
        }
    }
}
