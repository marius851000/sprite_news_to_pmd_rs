use crate::AllChanges;
use git2::{Commit, Repository, Revwalk, Sort, Tree};

fn get_commit<'repo>(repo: &'repo Repository, rev: &str) -> Commit<'repo> {
    repo.revparse_single(rev)
        .expect(&format!("can't find the latest revision {}", rev))
        .peel_to_commit()
        .expect(&format!("can't find the commit related to {}", rev))
}

fn push_revwalk(revwalk: &mut Revwalk, commit: &Commit) {
    revwalk.push(commit.id()).expect(&format!(
        "unable to add the revision {} to the revwalk",
        commit.id()
    ));
}

fn get_tree<'repo>(commit: &Commit<'repo>) -> Tree<'repo> {
    commit.tree().expect(&format!(
        "can't get the tree for the commit {}",
        commit.id()
    ))
}

pub fn get_changes_from_repo(
    repo: &Repository,
    latest_rev_id: &str,
    older_rev_id: &str,
) -> AllChanges {
    let mut revwalk = repo.revwalk().expect("can't get a revwalk from the repo");
    let latest_rev = get_commit(repo, latest_rev_id);
    let older_rev = get_commit(repo, older_rev_id);

    push_revwalk(&mut revwalk, &latest_rev);
    revwalk
        .set_sorting(Sort::TIME)
        .expect("can't set the sort mode of revwalk by time");

    let mut changes = AllChanges::default();

    for maybe_oid in revwalk {
        let oid = maybe_oid.expect("can't get a result of a revwalk");

        let commit = get_commit(repo, &oid.to_string());
        let commit_parent = commit.parent(0).expect(&format!(
            "can't get the unique parent of commit {}",
            commit.id()
        ));

        changes.add_diff_tree(repo, &get_tree(&commit_parent), &get_tree(&commit));

        if commit.id() == older_rev.id() {
            break;
        }
    }

    changes
}
