use git2::Oid;

use crate::credit::CreditEntry;

#[derive(Default, Debug)]
pub struct ChangeHistory {
    pub changes: Vec<Change>,
}

impl ChangeHistory {
    pub fn get_or_insert_mut(&mut self, id: &Oid, monster_name: String) -> &mut Change {
        for (change_id, change) in self.changes.iter().enumerate() {
            if &change.commit_ref == id {
                return self.changes.get_mut(change_id).unwrap();
            }
        }
        let new_position = self.changes.len();
        self.changes.push(Change::new(id.clone(), monster_name));
        return self.changes.get_mut(new_position).unwrap();
    }
}

#[derive(Debug)]
pub struct Change {
    pub author: Option<CreditEntry>,
    pub monster_name: String,
    pub commit_ref: Oid,
    pub portraits_change: KindChange<Vec<u8>>,
    pub sprites_change: KindChange<SpriteSheetContent>,
}

impl Change {
    fn new(commit_ref: Oid, monster_name: String) -> Self {
        Self {
            author: None,
            monster_name,
            commit_ref,
            portraits_change: KindChange::default(),
            sprites_change: KindChange::default(),
        }
    }
}

#[derive(Debug, Default)]
pub struct KindChange<T: PartialEq> {
    /// elem 1: name, elem 2: new content
    pub added: Vec<(String, T)>,
    /// elem 1: name, elem 2: old content, elem 3: new content
    pub changed: Vec<(String, T, T)>,
    /// elem 1: name, elem 2: old content
    pub removed: Vec<(String, T)>,
}

impl<T: PartialEq> KindChange<T> {
    pub fn have_change(&self) -> bool {
        !self.added.is_empty() || !self.changed.is_empty() || !self.removed.is_empty()
    }

    pub fn already_handled(&self, id: &str) -> bool {
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
