use serde::Deserialize;

#[derive(Deserialize, Debug)]
#[serde(rename_all = "PascalCase")]
pub struct AnimData {
    pub shadow_size: u32,
    pub anims: Anims
}

#[derive(Deserialize, Debug)]
pub struct Anims {
    #[serde(rename = "Anim")]
    pub anims: Vec<Anim>
}

impl Anims {
    pub fn get_item_by_name_follow_copy(&self, name: &str) -> Option<&Anim> {
        for anim in &self.anims {
            if anim.name == name {
                //TODO: actually follow copy
                return Some(anim)
            }
        }
        None
    }
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "PascalCase")]
pub struct Anim {
    pub name: String,
    pub frame_width: Option<u32>,
    pub frame_height: Option<u32>,
    pub durations: Option<Durations>
}

#[derive(Deserialize, Debug)]
pub struct Durations {
    #[serde(rename = "Duration")]
    pub durations: Vec<u32>
}

