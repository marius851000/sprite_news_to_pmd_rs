use std::collections::{BTreeSet, HashSet};

use git2::Oid;

use crate::{credit::CreditEntry, MonsterId};

#[derive(Debug)]
pub struct Change {
    pub authors: BTreeSet<CreditEntry>,
    pub monster_name: String,
    pub old_credit: Option<BTreeSet<CreditEntry>>,
    pub portraits_change: KindChange<Vec<u8>>,
    pub sprites_change: KindChange<SpriteSheetContent>,
}

impl Change {
    pub fn new(monster_name: String) -> Self {
        Self {
            authors: BTreeSet::new(),
            monster_name,
            old_credit: None,
            portraits_change: KindChange::default(),
            sprites_change: KindChange::default(),
        }
    }
}

#[derive(Debug, Default)]
pub struct KindChange<T> {
    /// elem 1: name, elem 2: new content
    pub added: Vec<(String, T)>,
    /// elem 1: name, elem 2: old content, elem 3: new content
    pub changed: Vec<(String, T, T)>,
    /// elem 1: name, elem 2: old content
    pub removed: Vec<(String, T)>,
}

impl<T> KindChange<T> {
    pub fn have_change(&self) -> bool {
        !self.added.is_empty() || !self.changed.is_empty() || !self.removed.is_empty()
    }

    fn already_handled(&self, id: &str) -> bool {
        for (name, _) in &self.added {
            if name == id {
                return true;
            }
        }
        for (name, _, _) in &self.changed {
            if name == id {
                return true;
            }
        }
        for (name, _) in &self.removed {
            if name == id {
                return true;
            }
        }
        false
    }

    /// Apply the given function on all element, returning a new KindChange keeping the same name and position.
    pub fn map<D, F>(&self, func: F) -> KindChange<D>
    where
        F: Fn(&T) -> D,
    {
        let mut dest: KindChange<D> = KindChange {
            added: Vec::new(),
            changed: Vec::new(),
            removed: Vec::new(),
        };

        for (name, elem) in &self.added {
            dest.added.push((name.clone(), func(elem)));
        }

        for (name, elem_old, elem_new) in &self.changed {
            dest.changed
                .push((name.clone(), func(elem_old), func(elem_new)));
        }

        for (name, elem) in &self.removed {
            dest.removed.push((name.clone(), func(elem)));
        }

        dest
    }
}

#[derive(PartialEq, Default, Debug)]
pub struct SpriteSheetContent {
    /// the content of *-Anim.png
    pub anim: Vec<u8>,
    /// the content of *-Offsets.png
    pub offsets: Vec<u8>,
    /// the content fo *-Shadow.png
    pub shadow: Vec<u8>,
    /// the name of the animation (that is, here, the content specified by *)
    pub name: String,
    /// the related AnimData.xml file
    pub animdata: Vec<u8>,
}
