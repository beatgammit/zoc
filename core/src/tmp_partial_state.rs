use std::collections::hash_map::{self, HashMap};
use unit::{Unit};
use map::{Map, Terrain};
use full_state::{FullState};
use game_state::{GameState, UnitIter};
use fow::{Fow};
use ::{
    PlayerId,
    UnitId,
    ObjectId,
    Object,
    Score,
    Sector,
    SectorId,
    ReinforcementPoints,
};

#[derive(Clone, Debug)]
pub struct TmpPartialState<'a> {
    state: &'a FullState,
    fow: &'a Fow,
}

impl<'a> TmpPartialState<'a> {
    pub fn new(
        state: &'a FullState,
        fow: &'a Fow,
    ) -> TmpPartialState<'a> {
        TmpPartialState {
            state: state,
            fow: fow,
        }
    }
}

impl<'a> GameState for TmpPartialState<'a> {
    type Fow = Fow;

    fn units2<'b>(&'b self) -> UnitIter<'b, Self::Fow> {
        UnitIter {
            iter: self.state.units(), // что делать, когда этот метод будет убран?
            fow: self.fow,
        }
    }

    fn units(&self) -> hash_map::Iter<UnitId, Unit> {
        self.state.units()
    }

    fn unit_opt(&self, id: UnitId) -> Option<&Unit> {
        self.state.unit_opt(id) // TODO: фильтровать леваков
    }

    fn objects(&self) -> &HashMap<ObjectId, Object> {
        self.state.objects()
    }

    fn map(&self) -> &Map<Terrain> {
        self.state.map()
    }

    fn sectors(&self) -> &HashMap<SectorId, Sector> {
        self.state.sectors()
    }

    fn score(&self) -> &HashMap<PlayerId, Score> {
        self.state.score()
    }

    fn reinforcement_points(&self) -> &HashMap<PlayerId, ReinforcementPoints> {
        self.state.reinforcement_points()
    }
}
