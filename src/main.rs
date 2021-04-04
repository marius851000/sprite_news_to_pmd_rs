use git2::Repository;
use sprite_news_presentation::{get_changes_from_repo, Output};

fn main() {
    let repo = Repository::open("/home/marius/SpriteCollab").expect("can't open the git reppo");
    let changes = get_changes_from_repo(
        &repo,
        "8db6d4f9e199db8dfe48d2dd0508564186d06d50",
        "62cf80450bd5e5d07a9beee40283e57cec6a21f7",
    );

    let o = Output::from_all_change(changes);
    o.write_to_folder("./test.md".into(), "./test_image".into());
}

// TODO: sprite support
