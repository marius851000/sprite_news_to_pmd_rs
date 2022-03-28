use git2::Repository;
use sprite_news_presentation::{get_changes_from_repo, Output};

fn main() {
    //TODO: only auto-hide shiny
    let repo = Repository::open("/home/marius/SpriteCollab").expect("can't open the git reppo");
    let changes = get_changes_from_repo(
        &repo,
        //week 57 - notspritecollab
        //"c3b51a3c626a4c2284e7b67274bca0f2e1adad51",
        //"42fa39605e0e833f60f08ab2cc59a8c01cd91447"
        
        //week 57 - spritecollab
        "ab8d86569b3c50ecc8331a6f8bffd938f64288f2",
        "83b733e0bed6c0a6b56b272cf9d9b9e573db490f"
        
        //week 56 - notspritecollab
        //"f9e2c93f96482dc4362be4d787fd4d18748fdadb",
        //"0421c4e11c067d68f07d87f3089aaec7bf2aac20"

        //week 56 - spritecollab
        //"6e316ebde88c4a1974699f732b83d78e13f17176",
        //"44035816d2f5ced2338bef9bead3de8b0ddd0fd3"

        //week 55 - spritecollab
        //"00a5ccdf62eabb0b8dff8e2df0c123baf54eee07",
        //"df51b713125527b4c99e6b7b2c7517dceb23b12e"

        //week 54 - notspritecollab
        //"994d609e9e74f6a7bbecee909efa6a88f3ed3acb",
        //"648dd991bf69a8dbf35f52c7a8acdc0c0b194262"
        
        //week 54
        //"4a93ccfa5a4b31517d33a45f3822fc5aff1f4830",
        //"545489a9b40ad950efafe30c102ba0ae4ec282c2"

        //week 50-53
        //"dc2b754da7cc81102f977b1fc201d5e5c16c64d0",
        //"795c7a6689194b8287040f5863d9c7c1003e12eb"

        //"9141d52cc91784f3371c3b6db8844031f145fbb9",
        //"f2ee0227b69cb46955721760276e14f84fd288ef"
        
        //"40dd27790eac5288d8393124d815ba135a2568ec",
        //"447949733907e267377c39bd9b4e6bf2a277ae3e"
        
        //"cc5dbad93576a9cfd4f2530e39f0ec056c1daa9e",
        //"eb8b910f2421ce30ed3be52fc81ffbed4e8a9e8e"
        
        //"ba8f7867b66c332f305fb80272985223e5868059",
        //"32fb6fb6f942de5752f3ec5e58b9132ab85b9ef8"
        
        //"787d3933b60f674a8715d1d112e1fc0d1286d5aa",
        //"d6dac58fd7d7f72958460b10478350ea6c5d1b89"

        //"cd26209f06c1b72ad0a6642a840fa457df544e19",
        //"7cb0e5a5a1f13e07f73a4d2ca229f925e00dae06"

        //"c1bdbbfe37d8e7624f9dad9a625897392e696b5c",
        //"ab618a8f56b3ac8ee803fa481f09247286e4c785"

        //"00a7ce0810f3f4a3259557616bdb9290f5940065",
        //"d163c00b3ff1e9ac73f0f20aeda7f680dc395677"

        //"8899d343d1bece70a3a8fa1bf7b08ebccfb7a858",
        //"df98c36a548336a944165a6a9f0e2615c00fa1c0"

        //"e6c897e2b7c6c285433eaa9d2fa644dff793e9ab", 
        //"7ec8d4a4929d9b11e883559cb642db52460481db"
        
        //"403d8a71a94f94c9355a9122cce4632b78a8631b",
        //"18b2ce890d9a1550c8289706483b9824d6f4161d"
        
        //"e052579b80d7a47b4cad3291c59affe539e9e804",
        //"26d33ef0c84376b567d9b0da16f1541b927c8bad"

        //"e1dc98fa6886f4001e5baa3bcb735fc2b56052ba",
        //"408c4c7b5d99d087f72484103d11cbcd301b2607"
        
        //"4bf1242e9b0eafaeae79253b9ce57a546b7ec023",
        //"792f01d6719e5ae28dcc11d58583c5203e20667b"
        
        //"db6eda96394f1c5bee06cfa569ca322786efc1ec",
        //"4e963a5efc7ce34ffca0db1974b1aebe1ea8b000"

        //"da4dc2788be13f9e6950790c89ef76ad0d0e2d4f",
        //"6447721ac97ea79dc65848a676702795742358f5"
        
        //"116e3d91c128bba62ecde7f0f24909ac52a514b6",
        //"9080e628c459b874eb212c6bfa252ad279665c6d"

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

    /*let repo = Repository::open("/home/marius/NotSpriteCollab").expect("can't open the git reppo");
    let changes = get_changes_from_repo(
        &repo,
        "86de4b08224587176d20f48b4e0c42c763ca6495",
        "090f46daa46bf781bd317e4213ceb5c0c5dd64b8"
    );*/

    let o = Output::from_all_change(changes);
    o.write_to_folder("./test.html".into(), "./changes/57-changes-spritecollab".into());
}
