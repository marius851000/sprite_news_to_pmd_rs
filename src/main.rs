use git2::Repository;
use sprite_news_presentation::{Output, get_changes_from_repo};

fn main() {
    let repo = Repository::open("/home/marius/SpriteCollab").expect("can't open the git reppo");
    let changes = get_changes_from_repo(
        &repo,
        "7833d2366437859fcafe20c14224c064c8b475d8",
        "3b419eba7309220d658766f556859b057bb77e44",
    );

    let o = Output::from_all_change(changes);
    o.write_to_folder("./test.md".into(), "./test_image".into());
}

// TODO: sprite support