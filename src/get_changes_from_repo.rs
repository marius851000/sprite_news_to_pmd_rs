use crate::AllChanges;
use git2::{Commit, Repository, Revwalk, Sort, Tree};

fn get_commit<'repo>(repo: &'repo Repository, rev: &str) -> Commit<'repo> {
    repo.revparse_single(rev)
        .unwrap_or_else(|_| panic!("can't find the latest revision {}", rev))
        .peel_to_commit()
        .unwrap_or_else(|_| panic!("can't find the commit related to {}", rev))
}
fn get_tree<'repo>(commit: &Commit<'repo>) -> Tree<'repo> {
    commit
        .tree()
        .unwrap_or_else(|_| panic!("can't get the tree for the commit {}", commit.id()))
}

pub fn get_changes_from_repo(
    repo: &Repository,
    latest_rev_id: &str,
    older_rev_id: &str,
) -> AllChanges {
    let latest_rev = get_commit(repo, latest_rev_id);
    let older_rev = get_commit(repo, older_rev_id);

    AllChanges::new_from_diff_in_tree(repo, &get_tree(&older_rev), &get_tree(&latest_rev))
}
