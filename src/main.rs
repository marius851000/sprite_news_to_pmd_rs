use git2::Repository;
use sprite_news_presentation::{get_changes_from_repo, Output};

fn main() {
    let repo = Repository::open("/home/marius/SpriteCollab").expect("can't open the git reppo");
    let changes = get_changes_from_repo(
        &repo,
        "2137714582c1509615dec89e3b1fdefd6e868e2d",
        "6495a78f5e97b2e042bb3bc617b7917103eedee3"
        //"b440cc0d4b8438898a1b66ba65ff935218646b0f",
        //"2f693b59209084462bf73ec20a19674c0d777d05",

        //"253dea526f135c310917d7e8bb2d4740f774a10b",
        //"4a90e104384fb954a2b15e8aaf3458ea3c35ed2b",
        
        //            "6f6b15ba83cb7f794a455e99542a9a5864c54802",
        //            "6b52850c9a993ba2162a97329dc6aa964efff2b3",

        //          "737d59e250be956554dd9058a5189e7c1f1da888",
        //          "f91593311f575944158f3194ac19286c43618633"

        //        "296caa4d52dc20ceb09c9c3384c94c87357f3307",
        //        "ed8ad4a4449c101b0248ed47022444f54b67b7e9",
    );

    let o = Output::from_all_change(changes);
    o.write_to_folder("./test.md".into(), "./images/11-changes".into());
}

// TODO: sprite support
