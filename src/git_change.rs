use git2::Oid;

use crate::credit::CreditEntry;

#[derive(Default, Debug)]
pub struct ChangeHistory {
    pub changes: Vec<Change>,
}

impl ChangeHistory {
    pub fn get_or_insert_mut(&mut self, id: &Oid) -> &mut Change {
        for (change_id, change) in self.changes.iter().enumerate() {
            if &change.commit_ref == id {
                return self.changes.get_mut(change_id).unwrap();
            }
        }
        let new_position = self.changes.len();
        self.changes.push(Change::new(id.clone()));
        return self.changes.get_mut(new_position).unwrap();
    }
}

#[derive(Debug)]
pub struct Change {
    pub author: Option<CreditEntry>,
    pub commit_ref: Oid,
    pub portraits_change: KindChange<Vec<u8>>,
    pub sprites_change: KindChange<SpriteSheetContent>,
}

impl Change {
    fn new(commit_ref: Oid) -> Self {
        Self {
            author: None,
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
}

#[derive(PartialEq, Default, Debug)]
pub struct SpriteSheetContent {
    /// the content of *-Anim.xml
    anim: Vec<u8>,
    /// the content of *-Offsets.xml
    offsets: Vec<u8>,
    /// the content fo *-Shadow.xml
    shadow: Vec<u8>,
    /// the name of the animation (that is, here, the content specified by *)
    name: String,
    /// the related AnimData.xml file
    animdata: Vec<u8>,
}
