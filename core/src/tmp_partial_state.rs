use std::collections::{HashMap};
use std::collections::hash_map;
use unit::{Unit};
use map::{Map, Terrain};
use internal_state::{InternalState};
use game_state::{GameState};
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
    state: &'a InternalState,
    fow: &'a Fow,
}

impl<'a> TmpPartialState<'a> {
    pub fn new(
        state: &'a InternalState,
        fow: &'a Fow,
    ) -> TmpPartialState<'a> {
        TmpPartialState {
            state: state,
            fow: fow,
        }
    }
}

impl<'a> GameState for TmpPartialState<'a> {
    fn units(&self) -> hash_map::Iter<UnitId, Unit> {
        // self.state.units().filter(|x| true)
        self.state.units()
    }

    fn unit_opt(&self, id: UnitId) -> Option<&Unit> {
        // TODO: фильтровать леваков
        self.state.unit_opt(id)
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
