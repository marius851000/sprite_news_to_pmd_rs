use git2::Repository;
use sprite_news_presentation::{get_changes_from_repo, Output};

fn main() {
    let repo = Repository::open("/home/marius/SpriteCollab").expect("can't open the git reppo");
    let changes = get_changes_from_repo(
        &repo,
        "9080e628c459b874eb212c6bfa252ad279665c6d",
        "9080e628c459b874eb212c6bfa252ad279665c6d"

        //"f578aea5dfb592a7a3767e034fabe2723d4a44ec",
        //"5bec7a69d665cd51abde161db8ec81509a754cb7"
        //"f1027ec6728ea5c31546fdcc833dfb0c7b06871a",
        //"a0ba39497633fae2233ac6a5b9db44706c6753f8"
        //"771ebb2865abd3775d199710b573d92c776e57a8",
        //"ba25bee506bb0ec6ae32a3ee2693ff709c34f420"

        //"71a0930993985d6c97a68714081f7aa73718eab6",
        //"d0432785b34eb1d259b3d3d6390417ebfb2caaf7"

        //"46905755e38c7dd4b2bc7121bebc74769a16c539",
        //"cdc0d166ad65254b5259e3e43a11ec6c34da8463"
        
        //"1c2efed111c1f31f157e573ea3e4feb62be8f591",
        //"d8c862abef79e6fd4688ab1f1fd2433c76ec16fe"

        //"64f05f38d32232b441e3c5fa9b90b26947462828",
        //"126904fde4c09461b6a75fe321f1231cfa86a035"

//        "68922acddbfba70cb4f927cee820edb77ae13cbd",
//        "c18e3cb8d0888ba0b6eddb5e20bdd52e5cfef632"

        //"f41703a043716b57f2f3804c528bf60507910066",
        //"84d1ed61ede968216bed871e2f23e434ccbc50bb"

        //"2bbfbd4ccfdbf5e19949ec0a4fd5b423cdb18af3",
        //"54a8db1a7588ab2751ce59139112bf3490384d77"
        
        //"0ef2d7dc9b286640960de8ffaa5965d10c45f7dc",
        //"fcc203ea8cc62808ffea4466bdf4565a66f09030"
        
        //"731aeaf8b957702b3905e279cb8e9a712fd0bfc4",
        //"b37ec5ee90d29fa092ca38ae839168f66506ece3",
        
        //"4af0d7b6f7b32195ad8f6d2079b6b3e2eef6a097",
        //"fa6a78dc40e853f431b82c356001f03133a10fc9",

        //"a3c728fd5ee5a31ae600e0a8c8d168cc02d56588",
        //"021216bcb339653c94403eff9b7c6d441f9c7432"
        //"c273ca0d094c61dde306ceedf269edfd22627a62",
        //"92de7f23b29622d87d6af22c97cf260b5186147a",

        //"3c88e0d2762f3fa3fbe7d8fa4f814382f9eac11f",
        //"251f41c244ed2a238b144e4afe12f97d9795d8cb",
        //"2137714582c1509615dec89e3b1fdefd6e868e2d",
        //"6495a78f5e97b2e042bb3bc617b7917103eedee3"
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
    o.write_to_folder("./test.html".into(), "./images/30-changes".into());
}

// TODO: sprite support
