use std::collections::HashMap;

use serde::Deserialize;

use crate::MonsterId;

#[derive(Deserialize, Debug)]
pub struct Tracker(HashMap<String, TrackerMonsterEntry>);

impl Tracker {
    pub fn get_subgroup(&self, monster_id: &MonsterId) -> &TrackerMonsterEntry {
        let mut part_iter = monster_id.path.iter();
        let mut current = self.0.get(part_iter.next().unwrap()).unwrap();
        for part in part_iter {
            current = current.subgroups.0.get(part).unwrap();
        };
        current
    }
}

#[derive(Deserialize, Debug)]
pub struct TrackerMonsterEntry {
    pub name: String,
    pub portrait_credit: String,
    pub sprite_credit: String,
    pub subgroups: Box<Tracker>,
}