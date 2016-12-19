use std::collections::hash_map::{self, HashMap};
use unit::{Unit};
use map::{Map, Terrain};
use fow::{FogOfWar};
use ::{
    CoreEvent,
    UnitId,
    ObjectId,
    Object,
    MapPos,
    ExactPos,
    Sector,
    SectorId,
    PlayerId,
    Score,
    ReinforcementPoints,
};

#[derive(Clone)]
pub struct ObjectsAtIter<'a> {
    it: hash_map::Iter<'a, ObjectId, Object>,
    pos: MapPos,
}

impl<'a> ObjectsAtIter<'a> {
    pub fn new(objects: &HashMap<ObjectId, Object>, pos: MapPos) -> ObjectsAtIter {
        ObjectsAtIter{it: objects.iter(), pos: pos}
    }
}

impl<'a> Iterator for ObjectsAtIter<'a> {
    type Item = &'a Object;

    fn next(&mut self) -> Option<Self::Item> {
        while let Some((_, object)) = self.it.next() {
            for map_pos in object.pos.map_pos_iter() {
                if self.pos == map_pos {
                    return Some(object);
                }
            }
        }
        None
    }
}

#[derive(Clone)]
pub struct UnitsAtIter<'a, Fow: FogOfWar + 'a, S: GameState + 'a> {
    it: UnitIter<'a, Fow, S>,
    pos: MapPos,
}

impl<'a, Fow: FogOfWar + 'a, S: GameState + 'a> Iterator for UnitsAtIter<'a, Fow, S> {
    type Item = &'a Unit;

    fn next(&mut self) -> Option<Self::Item> {
        while let Some((_, unit)) = self.it.next() {
            if self.pos == unit.pos.map_pos {
                return Some(unit);
            }
        }
        None
    }
}

pub trait GameState: Sized + Clone {
    type Fow: FogOfWar;

    fn map(&self) -> &Map<Terrain>;

    fn units(&self) -> hash_map::Iter<UnitId, Unit>;

    fn units2<'a>(&'a self) -> UnitIter<'a, Self::Fow, Self>;

    fn unit_opt(&self, id: UnitId) -> Option<&Unit>;

    fn objects(&self) -> &HashMap<ObjectId, Object>;
    fn sectors(&self) -> &HashMap<SectorId, Sector>;
    fn score(&self) -> &HashMap<PlayerId, Score>;
    fn reinforcement_points(&self) -> &HashMap<PlayerId, ReinforcementPoints>;

    fn unit(&self, id: UnitId) -> &Unit {
        self.unit_opt(id).unwrap()
    }

    fn units_at(&self, pos: MapPos) -> UnitsAtIter<Self::Fow, Self> {
        UnitsAtIter{it: self.units2(), pos: pos}
    }

    fn unit_at_opt(&self, pos: ExactPos) -> Option<&Unit> {
        for unit in self.units_at(pos.map_pos) {
            if unit.pos == pos {
                return Some(unit);
            }
        }
        None
    }

    fn unit_at(&self, pos: ExactPos) -> &Unit {
        self.unit_at_opt(pos).unwrap()
    }

    fn objects_at(&self, pos: MapPos) -> ObjectsAtIter {
        ObjectsAtIter::new(self.objects(), pos)
    }
}

pub trait GameStateMut: GameState {
    fn apply_event(&mut self, event: &CoreEvent);
}

#[derive(Clone)]
pub struct UnitIter<'a, Fow: FogOfWar + 'a, S: GameState + 'a> {
    pub iter: hash_map::Iter<'a, UnitId, Unit>,
    pub fow: &'a Fow,
    pub state: &'a S,
}

impl<'a, S: GameState, Fow: FogOfWar> Iterator for UnitIter<'a, Fow, S> {
    type Item = (&'a UnitId, &'a Unit);

    fn next(&mut self) -> Option<Self::Item> {
        while let Some(pair) = self.iter.next() {
            let (_, unit) = pair;
            if self.fow.is_visible(self.state, unit, unit.pos) {
                println!("Noooo?");
                return Some(pair);
            } else {
                println!("YYEESSS");
            }
        }
        None
    }
}
