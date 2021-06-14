use std::{
    collections::HashSet,
    fs::{create_dir_all, File},
    path::PathBuf,
};

use image::{DynamicImage, ImageFormat};

use crate::sprite::SpriteSheet;

pub struct ImageStore {
    used_name: HashSet<String>,
    folder: PathBuf,
}

impl ImageStore {
    pub fn new(folder: PathBuf) -> Self {
        create_dir_all(&folder).unwrap();
        Self {
            used_name: HashSet::new(),
            folder,
        }
    }

    pub fn reserve_file_with_extension(&mut self, name_tip: &str, extension: &str) -> PathBuf {
        let name_tip = if self.used_name.contains(name_tip) {
            let mut id_addition = 0;
            loop {
                let new_name_tip = format!("{}-{}", name_tip, id_addition);
                if self.used_name.contains(&new_name_tip) {
                    id_addition += 1;
                } else {
                    break new_name_tip;
                }
            }
        } else {
            name_tip.to_string()
        };
        self.used_name.insert(name_tip.to_string());
        let filename = format!("{}{}", name_tip, extension);
        self.folder.join(filename)
    }

    pub fn add_image(&mut self, image: DynamicImage, name_tip: &str) -> PathBuf {
        let target_file = self.reserve_file_with_extension(name_tip, ".png");
        image
            .save_with_format(&target_file, ImageFormat::Png)
            .unwrap();
        target_file
    }

    pub fn add_spritesheet(&mut self, sprite_sheet: &SpriteSheet, name_tip: &str) -> PathBuf {
        let target_file = self.reserve_file_with_extension(name_tip, ".png");
        let mut writer = File::create(&target_file).unwrap();
        sprite_sheet.write_apng(&mut writer);
        target_file
    }
}
