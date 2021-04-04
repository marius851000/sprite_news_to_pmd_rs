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

    pub fn get_monster_name(&self, monster_id: &MonsterId) -> String {
        let mut part_iter = monster_id.path.iter();
        let mut current = self.0.get(part_iter.next().unwrap()).unwrap();
        let mut result = current.name.clone();
        for part in part_iter {
            current = current.subgroups.0.get(part).unwrap();
            result.push(' ');
            result.push_str(&current.name);
        };
        result
    }
}

#[derive(Deserialize, Debug)]
pub struct TrackerMonsterEntry {
    pub name: String,
    pub portrait_credit: String,
    pub sprite_credit: String,
    pub subgroups: Box<Tracker>,
}