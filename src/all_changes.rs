use std::{
    collections::{BTreeMap, HashMap},
    path::{Path, PathBuf},
};

use git2::{Delta, Oid, Repository, Tree};

use crate::{git_change::SpriteSheetContent, ChangeHistory, Credit, MonsterId, Tracker};

#[derive(Default, Debug)]
pub struct AllChanges {
    pub changes: BTreeMap<MonsterId, ChangeHistory>,
}

impl AllChanges {
    pub fn add_diff_tree(&mut self, repo: &Repository, old_tree: &Tree, new_tree: &Tree) {
        let diff = repo
            .diff_tree_to_tree(Some(old_tree), Some(&new_tree), None)
            .unwrap();

        let credit = Credit::new_from_file(
            &String::from_utf8(
                repo.find_blob(
                    new_tree
                        .get_path(&PathBuf::from("credit_names.txt"))
                        .unwrap()
                        .id(),
                )
                .unwrap()
                .content()
                .to_vec(),
            )
            .unwrap(),
        );

        let mut tracker_cache: HashMap<Oid, Tracker> = HashMap::new();

        for delta in diff.deltas() {
            let (reference_file, reference_tree) = if delta.status() == Delta::Deleted {
                (delta.old_file(), old_tree)
            } else {
                (delta.new_file(), new_tree)
            };

            let tracker = if let Some(tracker) = tracker_cache.get(&reference_tree.id()) {
                tracker
            } else {
                let tracker: Tracker = serde_json::from_slice(
                    repo.find_blob(
                        reference_tree
                            .get_path(&PathBuf::from("tracker.json"))
                            .unwrap()
                            .id(),
                    )
                    .unwrap()
                    .content(),
                )
                .unwrap();
                tracker_cache.insert(reference_tree.id(), tracker);
                tracker_cache.get(&reference_tree.id()).unwrap()
            };

            let path = reference_file
                .path()
                .expect(&format!("can't get path for {:?}", reference_file));

            let change_is_on = path
                .iter()
                .next()
                .expect(&format!(
                    "can't get the first part of the path of {:?}",
                    reference_file
                ))
                .to_str()
                .expect(&format!(
                    "can't get convert the of the file {:?}",
                    reference_file
                ));

            let file_name = path.iter().last().unwrap().to_string_lossy().to_string();

            if file_name == "credits.txt" {
                continue;
            };

            match change_is_on {
                "portrait" | "sprite" => {
                    let mut changed_pokemon_path = path.iter().skip(1).collect::<PathBuf>();
                    changed_pokemon_path.pop();
                    let monster_id = MonsterId::from_path(&changed_pokemon_path);

                    let tracker_entry = tracker.get_subgroup(&monster_id);
                    let monster_name = tracker.get_monster_name(&monster_id);

                    let changed_monster = if let Some(v) = self.changes.get_mut(&monster_id) {
                        v
                    } else {
                        self.changes
                            .insert(monster_id.clone(), ChangeHistory::default());
                        self.changes.get_mut(&monster_id).unwrap()
                    };

                    let oid = reference_tree.id();
                    let change = changed_monster.get_or_insert_mut(&oid, monster_name);

                    let changed_content_name = file_name.split(".").next().unwrap().to_string();

                    match change_is_on {
                        "portrait" => {
                            if tracker_entry.portrait_credit == "" {
                                change.author = None;
                            } else {
                                change.author = Some(credit.get(&tracker_entry.portrait_credit));
                            }
                            let portrait_file = repo
                                .find_blob(reference_file.id())
                                .expect("can't get a portrait blob")
                                .content()
                                .to_vec();
                            match delta.status() {
                                Delta::Deleted => change
                                    .portraits_change
                                    .removed
                                    .push((changed_content_name, portrait_file)),
                                Delta::Added => change
                                    .portraits_change
                                    .added
                                    .push((changed_content_name, portrait_file)),
                                Delta::Modified => {
                                    let old_file = repo
                                        .find_blob(delta.old_file().id())
                                        .unwrap()
                                        .content()
                                        .to_vec();
                                    change.portraits_change.changed.push((
                                        changed_content_name,
                                        old_file,
                                        portrait_file,
                                    ));
                                }
                                _ => todo!(),
                            }
                        }
                        "sprite" => {
                            if changed_content_name == "AnimData" || changed_content_name == "FrameData" {
                                continue;
                            };
                            let changed_anim_name = changed_content_name.split('-').next().unwrap();
                            if change.sprites_change.already_handled(changed_anim_name) {
                                continue;
                            }

                            if tracker_entry.sprite_credit == "" {
                                change.author = None;
                            } else {
                                change.author = Some(credit.get(&tracker_entry.sprite_credit));
                            }

                            let reference_sprite = get_sprite_sheet_from_tree(
                                &repo,
                                &reference_tree,
                                &path.parent().unwrap(),
                                changed_anim_name,
                            );

                            match delta.status() {
                                Delta::Deleted => change
                                    .sprites_change
                                    .removed
                                    .push((changed_anim_name.to_string(), reference_sprite)),
                                Delta::Added => change
                                    .sprites_change
                                    .added
                                    .push((changed_anim_name.to_string(), reference_sprite)),
                                Delta::Modified => {
                                    let old_sprite = get_sprite_sheet_from_tree(
                                        &repo,
                                        &old_tree,
                                        &path.parent().unwrap(),
                                        changed_anim_name,
                                    );
                                    change.sprites_change.changed.push((
                                        changed_anim_name.to_string(),
                                        old_sprite,
                                        reference_sprite,
                                    ));
                                }
                                _ => todo!(),
                            }
                        }
                        _ => panic!(),
                    };
                }
                "tracker.json" | "credit_names.txt" => {}
                root_folder => panic!("unknown root file/folder: {:?}", root_folder),
            }
        }
    }
}

pub fn get_sprite_sheet_from_tree(
    repo: &Repository,
    tree: &Tree,
    path: &Path,
    name: &str,
) -> SpriteSheetContent {
    let get_file = |name: &str| {
        repo.find_blob(tree.get_path(&path.join(name)).unwrap().id())
            .unwrap()
            .content()
            .to_vec()
    };

    let anim = get_file(&format!("{}-Anim.png", name));
    let offsets = get_file(&format!("{}-Offsets.png", name));
    let shadow = get_file(&format!("{}-Shadow.png", name));
    let animdata = get_file("AnimData.xml");

    SpriteSheetContent {
        anim,
        offsets,
        shadow,
        name: name.to_string(),
        animdata,
    }
}
