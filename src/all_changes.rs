use std::{
    collections::{BTreeMap, BTreeSet, HashMap},
    path::{Path, PathBuf},
};

use git2::{Commit, Delta, Oid, Repository, Tree};

use crate::{
    credit::CreditEntry, git_change::SpriteSheetContent, tracker::Credit, Change, GlobalCredit,
    MonsterId, SingleMonsterCredit, Tracker,
};

#[derive(Default, Debug)]
pub struct AllChanges {
    pub changes: BTreeMap<MonsterId, Change>,
}

impl AllChanges {
    pub fn get_or_insert_mut(
        &mut self,
        monster_id: MonsterId,
        monster_name: String,
    ) -> &mut Change {
        let entry = self
            .changes
            .entry(monster_id)
            .or_insert_with(move || Change::new(monster_name));
        entry
    }
}

fn get_blob(repo: &Repository, tree: &Tree, path: &Path) -> Option<Vec<u8>> {
    if let Ok(r) = repo.find_blob(tree.get_path(path).unwrap().id()) {
        Some(r.content().to_vec())
    } else {
        None
    }
}

impl AllChanges {
    pub fn new_from_diff_in_tree(repo: &Repository, old_tree: &Tree, new_tree: &Tree) -> Self {
        let mut result = Self {
            changes: BTreeMap::new(),
        };

        let diff = repo
            .diff_tree_to_tree(Some(old_tree), Some(&new_tree), None)
            .unwrap();

        let new_global_credit = GlobalCredit::new_from_file(
            &String::from_utf8(
                get_blob(&repo, &new_tree, &PathBuf::from("credit_names.txt")).unwrap(),
            )
            .unwrap(),
        );

        let old_global_credit = GlobalCredit::new_from_file(
            &String::from_utf8(
                get_blob(&repo, &old_tree, &PathBuf::from("credit_names.txt")).unwrap(),
            )
            .unwrap(),
        );

        let new_tracker: Tracker = serde_json::from_slice(
            &get_blob(&repo, &new_tree, &PathBuf::from("tracker.json")).unwrap(),
        )
        .unwrap();
        let old_tracker: Tracker = serde_json::from_slice(
            &get_blob(&repo, &old_tree, &PathBuf::from("tracker.json")).unwrap(),
        )
        .unwrap();

        for delta in diff.deltas() {
            let (reference_file, reference_tree) = if delta.status() == Delta::Deleted {
                (delta.old_file(), old_tree)
            } else {
                (delta.new_file(), new_tree)
            };

            let path = reference_file
                .path()
                .unwrap_or_else(|| panic!("can't get path for {:?}", reference_file));

            let change_is_on = path
                .iter()
                .next()
                .unwrap_or_else(|| {
                    panic!(
                        "can't get the first part of the path of {:?}",
                        reference_file
                    )
                })
                .to_str()
                .unwrap_or_else(|| {
                    panic!("can't get convert the of the file {:?}", reference_file)
                });

            let file_name = path.iter().last().unwrap().to_string_lossy().to_string();

            //TODO: handle crediting changes
            match change_is_on {
                "portrait" | "sprite" => {
                    //TODO: this is done to ensure that no animation change get credited thrice. Should be changed to a proper format, that check if the change has already been added previously
                    let mut change_folder = path.to_path_buf();          
                    change_folder.pop();

                    let mut changed_pokemon_path = path.iter().skip(1).collect::<PathBuf>();
                    changed_pokemon_path.pop();

                    let monster_id = MonsterId::from_path(&changed_pokemon_path);

                    let oid = reference_tree.id();

                    let (tracker_to_use, monster_name) =
                        if let Some(name) = new_tracker.get_monster_name(&monster_id) {
                            (&new_tracker, name)
                        } else if let Some(name) = old_tracker.get_monster_name(&monster_id) {
                            (&old_tracker, name)
                        } else {
                            println!(
                                "Can't find the monster with the id {:?}, present in tree id {}",
                                monster_id, oid
                            );
                            continue;
                        };

                    // CREDIT HANDLING

                    let change = result.get_or_insert_mut(monster_id, monster_name);

                    if change.authors.is_empty() {
                        // use the newest credit if avalaible. If it isn't in the newest credit, then it has been deleted.
                        let (monster_credit_to_use, global_credit_to_use) =
                            if let Some(new_monster_credit_blob) =
                                get_blob(repo, &reference_tree, &change_folder.join("credits.txt"))
                            {
                                (
                                    SingleMonsterCredit::new_from_file(
                                        &String::from_utf8(new_monster_credit_blob).unwrap(),
                                    ),
                                    &new_global_credit,
                                )
                            } else {
                                let monster_credit_blob =
                                    get_blob(repo, &old_tree, &change_folder.join("credits.txt"))
                                        .unwrap();
                                (
                                    SingleMonsterCredit::new_from_file(
                                        &String::from_utf8(monster_credit_blob).unwrap(),
                                    ),
                                    &old_global_credit,
                                )
                            };

                        for author_id in &monster_credit_to_use.credits {
                            change.authors.insert(
                                if let Some(author) = global_credit_to_use.entries.get(author_id) {
                                    author.clone()
                                } else {
                                    CreditEntry::new(None, None, author_id.clone())
                                },
                            );
                        }
                    }

                    if path
                        .file_name()
                        .expect("can't get the filename for a portrait/sprite change")
                        == "credits.txt"
                    {
                        continue; //TODO: implement credits change
                    }

                    let changed_content_name = file_name.split('.').next().unwrap().to_string();

                    match change_is_on {
                        "portrait" => {
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
                            if changed_content_name == "AnimData"
                                || changed_content_name == "FrameData"
                            {
                                continue;
                            };

                            //TODO: that's ugly
                            let mut iter_for_change_kind = changed_content_name.split("-");
                            iter_for_change_kind.next().unwrap();
                            if "Anim" != iter_for_change_kind.next().unwrap() {
                                continue
                            }

                            let changed_anim_name = changed_content_name.split('-').next().unwrap();

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
                "tracker.json" | "credit_names.txt" | "README.md" | "sprite_config.json" => {}
                root_folder => panic!("unknown root file/folder: {:?}", root_folder),
            }
        }

        result
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
