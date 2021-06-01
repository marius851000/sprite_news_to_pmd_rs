use image::{load_from_memory_with_format, DynamicImage, ImageBuffer, ImageFormat, Rgb, Rgba};
use std::{collections::HashSet, fs::create_dir_all, io::Write};
use std::{fs::File, path::PathBuf};

use crate::{
    present_portrait_picture, AllChanges, Change, KindChange, MonsterId,
    PortraitPicturePresentation,
};

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

    pub fn add_image(&mut self, image: DynamicImage, name_tip: &str) -> PathBuf {
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
        let filename = format!("{}.png", name_tip);
        let target_file = self.folder.join(filename);
        image
            .save_with_format(&target_file, ImageFormat::Png)
            .unwrap();
        target_file
    }
}

#[derive(Debug)]
pub struct Output {
    pub out: Vec<OutputItem>,
}

impl Output {
    pub fn from_all_change(changes: AllChanges) -> Self {
        let mut out = Vec::new();
        for (monster_id, change_history) in changes.changes.iter() {
            for change in &change_history.changes {
                out.extend(OutputItem::from_change(monster_id, change));
            }
        }
        Self { out }
    }

    pub fn write_to_folder(&self, text_path: PathBuf, image_path: PathBuf) {
        let mut image_store = ImageStore::new(image_path);

        let mut result_markdown_file = String::new();
        for output in &self.out {
            result_markdown_file.push_str("- ");
            result_markdown_file.push_str(&output.label);
            result_markdown_file.push_str("\n\n");
            for illustration in &output.illustrations {
                illustration.write(&mut result_markdown_file, &mut image_store);
            }
        }

        let mut text_writer = File::create(&text_path).unwrap();
        text_writer
            .write_all(result_markdown_file.as_bytes())
            .unwrap();
    }
}

#[derive(Debug)]
pub struct OutputItem {
    pub label: String,
    // second element is tip for file name
    pub illustrations: Vec<ChangeIllustration>,
}

impl OutputItem {
    fn from_change(monster_id: &MonsterId, change: &Change) -> Vec<Self> {
        let mut result = Vec::new();
        if change.portraits_change.have_change() {
            let text = generate_text("portrait", "portraits", &change.portraits_change, &change);

            let presentation = PortraitPicturePresentation {
                additions: change
                    .portraits_change
                    .added
                    .iter()
                    .map(|(_, x)| load_png_from_mem(x))
                    .collect(),
                modifications: change
                    .portraits_change
                    .changed
                    .iter()
                    .map(|(_, x, y)| (load_png_from_mem(x), load_png_from_mem(y)))
                    .collect(),
                deletions: change
                    .portraits_change
                    .removed
                    .iter()
                    .map(|(_, x)| load_png_from_mem(x))
                    .collect(),
            };

            let illustrations = present_portrait_picture(presentation, monster_id.to_slug());

            result.push(OutputItem {
                illustrations,
                label: text,
            })
        };

        if change.sprites_change.have_change() {
            let text = generate_text(
                "sprite kind",
                "sprites kinds",
                &change.sprites_change,
                &change,
            );

            result.push(OutputItem {
                illustrations: Vec::new(),
                label: text,
            });
        }

        result
    }
}

#[derive(Debug)]
pub struct ChangeIllustration {
    pub filename_tip: String,
    pub title: String,
    pub image: ChangeIllustrationImage,
}

impl ChangeIllustration {
    pub fn write(&self, md: &mut String, img_store: &mut ImageStore) {
        md.push_str("  - **");
        md.push_str(&self.title);
        md.push_str("**");
        md.push_str("\n\n");
        let image = match &self.image {
            ChangeIllustrationImage::Portraits(image) => DynamicImage::ImageRgba8(image.clone()),
        };
        let image_path = img_store.add_image(image, &self.filename_tip);
        md.push_str("![](");
        md.push_str(image_path.to_str().unwrap());
        md.push_str(")\n\n");
    }
}

#[derive(Debug)]
pub enum ChangeIllustrationImage {
    Portraits(ImageBuffer<Rgba<u8>, Vec<u8>>),
}

fn generate_text<T: PartialEq>(
    singular: &str,
    plurial: &str,
    kind: &KindChange<T>,
    change: &Change,
) -> String {
    let author = &change.author;
    let author_text = if let Some(author) = author {
        let author_description = if let Some(name) = &author.name {
            name.to_string()
        } else {
            format!("Someone with the (discord) id {}", author.id)
        };
        if let Some(contact) = &author.contact {
            format!("[{}]({})", author_description, contact)
        } else {
            author_description
        }
    } else {
        "someone".to_string()
    };

    let mut change_vec = Vec::new();
    if !kind.added.is_empty() {
        change_vec.push(gen_change_text(
            "added",
            kind.added.iter().map(|(x, _)| x.to_string()).collect(),
        ));
    }
    if !kind.removed.is_empty() {
        change_vec.push(gen_change_text(
            "removed",
            kind.removed.iter().map(|(x, _)| x.to_string()).collect(),
        ));
    }
    if !kind.changed.is_empty() {
        change_vec.push(gen_change_text(
            "changed",
            kind.changed.iter().map(|(x, _, _)| x.to_string()).collect(),
        ));
    }

    let change_vec_len = change_vec.len();
    let change_text = human_list(change_vec);

    let what_change_text = if change_vec_len == 1 {
        singular
    } else {
        plurial
    };

    format!(
        "{} {} {} for {}",
        author_text, change_text, what_change_text, change.monster_name
    ) //TODO: link to commit
}

fn gen_change_text(verb: &str, changes: Vec<String>) -> String {
    if changes.len() <= 5 {
        format!("{} the {}", verb, human_list(changes))
    } else {
        format!("{} {}", verb, changes.len())
    }
}

fn human_list(elements: Vec<String>) -> String {
    if elements.is_empty() {
        String::new()
    } else if elements.len() == 1 {
        elements.first().unwrap().to_string()
    } else {
        let mut result = String::new();
        for (pos, change) in elements.iter().enumerate() {
            let pos_to_last = elements.len() - pos - 1;
            if pos == 0 {
                result = change.to_string()
            } else if pos_to_last == 0 {
                result.push_str(" and ");
                result.push_str(change);
            } else {
                result.push_str(", ");
                result.push_str(change);
            }
        }
        result
    }
}

fn load_png_from_mem(i: &[u8]) -> ImageBuffer<Rgb<u8>, Vec<u8>> {
    load_from_memory_with_format(i, ImageFormat::Png)
        .unwrap()
        .into_rgb8()
}
