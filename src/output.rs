use image::{load_from_memory_with_format, ImageBuffer, ImageFormat, Rgb};
use std::io::Write;
use std::{fs::File, path::PathBuf};

use crate::{
    present_portrait_picture, AllChanges, Change, KindChange, MonsterId,
    PortraitPicturePresentation,
};

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
        let mut image_next_id = 0;
        let mut result_markdown_file = String::new();
        for output in &self.out {
            result_markdown_file.push_str("- ");
            result_markdown_file.push_str(&output.text);
            result_markdown_file.push('\n');
            if let Some((image, image_name_tip)) = &output.image {
                let image_id = image_next_id;
                image_next_id += 1;
                let image_path = image_path.join(format!("{}-{}.png", image_name_tip, image_id));
                image
                    .save_with_format(&image_path, ImageFormat::Png)
                    .unwrap();
                result_markdown_file.push_str(&format!("\n![]({})\n", image_path.display()));
            };
        }
        let mut text_writer = File::create(&text_path).unwrap();
        text_writer
            .write_all(result_markdown_file.as_bytes())
            .unwrap();
    }
}

#[derive(Debug)]
pub struct OutputItem {
    // second element is tip for file name
    pub image: Option<(ImageBuffer<Rgb<u8>, Vec<u8>>, String)>,
    pub text: String,
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

            let image = present_portrait_picture(presentation);

            result.push(OutputItem {
                image: Some((image, monster_id.path.join("-").replace('/', "_"))),
                text,
            })
        };

        if change.sprites_change.have_change() {
            let text = generate_text("sprite kind", "sprites kinds", &change.sprites_change, &change);

            result.push(OutputItem { image: None, text });
        }

        result
    }
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
    if elements.len() == 0 {
        return String::new();
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
