use image::{load_from_memory_with_format, ImageFormat};
use maud::{html, Markup};

use std::collections::BTreeMap;

use std::io::Write;
use std::path::Path;
use std::{fs::File, path::PathBuf};

use crate::credit::CreditEntry;
use crate::{sprite::SpriteSheet, AllChanges, Change};
use crate::{ImageStore, Portrait};

fn embed_image(path: &Path) -> Markup {
    html! {
        img src=(path.to_string_lossy()) {}
    }
}

#[derive(Debug)]
pub struct Output {
    pub out: BTreeMap<(String, Option<CreditEntry>), Vec<OutputItem>>,
}

impl Output {
    pub fn from_all_change(changes: AllChanges) -> Self {
        let mut out: BTreeMap<(String, Option<CreditEntry>), Vec<OutputItem>> = BTreeMap::new();

        for (_monster_id, change_history) in changes.changes.iter() {
            for change in &change_history.changes {
                let author = &change.author;
                let identifier_pair = (change.monster_name.clone(), author.clone());
                let output_item = OutputItem::from_change(&change.monster_name, change);
                if let Some(already_present) = out.get_mut(&identifier_pair) {
                    already_present.extend(output_item);
                } else {
                    out.insert(identifier_pair, output_item);
                }
            }
        }
        Self { out }
    }

    pub fn write_to_folder(&self, text_path: PathBuf, image_path: PathBuf) {
        let mut image_store = ImageStore::new(image_path);

        /*let mut result_markdown_file = String::new();
        for output in &self.out {
            result_markdown_file.push_str("- ");
            // result_markdown_file.push_str(&output.label);
            result_markdown_file.push_str("\n\n");
            for illustration in &output.illustrations {
                illustration.write(&mut result_markdown_file, &mut image_store);
            }
        }*/

        let html = self.render_html(&mut image_store);

        let mut text_writer = File::create(&text_path).unwrap();
        text_writer
            .write_all(html.into_string().as_bytes())
            .unwrap();
    }

    pub fn render_html(&self, image_store: &mut ImageStore) -> Markup {
        html!(
            @for (key, sections) in &self.out {
                details {
                    summary {
                        //TODO: get the good name
                        b { (key.0) }
                        @if let Some(author) = &key.1 {
                            " by " (author.render_html())
                        }
                    }
                    @for section in sections {
                        (section.render_html(image_store))
                    }
                }
            }
        )
    }
}

#[derive(Debug)]
pub struct OutputItem {
    pub label: String,
    // second element is tip for file name
    pub illustrations: ChangeIllustrations,
}

fn decode_portrait(binary: &[u8]) -> Portrait {
    Portrait(
        load_from_memory_with_format(binary, ImageFormat::Png)
            .unwrap()
            .into_rgba8(),
    )
}

impl OutputItem {
    fn from_change(_monster_name: &str, change: &Change) -> Vec<Self> {
        //TODO: use the monster id for the image name
        //TODO: plurial
        let mut result = Vec::new();

        for (label, content) in &[
            ("portrait added", &change.portraits_change.added),
            ("portrait removed", &change.portraits_change.removed),
        ] {
            if !content.is_empty() {
                result.push(OutputItem {
                    label: label.to_string(),
                    illustrations: ChangeIllustrations::PortraitSingle(
                        content
                            .into_iter()
                            .map(|(name, img_binary)| {
                                (name.to_string(), decode_portrait(img_binary))
                            })
                            .collect(),
                    ),
                })
            }
        }

        if !change.portraits_change.changed.is_empty() {
            result.push(OutputItem {
                label: "portrait changed".to_string(),
                illustrations: ChangeIllustrations::PortraitModification(
                    change
                        .portraits_change
                        .changed
                        .iter()
                        .map(|(label, old, new)| {
                            (
                                label.to_string(),
                                decode_portrait(old),
                                decode_portrait(new),
                            )
                        })
                        .collect(),
                ),
            })
        };

        //TODO: plurial
        for (label, content) in &[
            ("sprite removed", &change.sprites_change.removed),
            ("sprite added", &change.sprites_change.added),
        ] {
            if !content.is_empty() {
                result.push(OutputItem {
                    label: label.to_string(),
                    illustrations: ChangeIllustrations::SpriteSingle(
                        content
                            .iter()
                            .map(|(label, spr_sheet_content)| {
                                (
                                    label.to_string(),
                                    SpriteSheet::new_from_change(spr_sheet_content),
                                )
                            })
                            .collect(),
                    ),
                })
            }
        }

        //TODO: plurial again
        if !change.sprites_change.changed.is_empty() {
            result.push(OutputItem {
                label: "sprite changed".to_string(),
                illustrations: ChangeIllustrations::SpriteModification(
                    change
                        .sprites_change
                        .changed
                        .iter()
                        .map(|(label, old, new)| {
                            (
                                label.to_string(),
                                SpriteSheet::new_from_change(old),
                                SpriteSheet::new_from_change(new),
                            )
                        })
                        .collect(),
                ),
            })
        }

        result
    }

    pub fn render_html(&self, img_store: &mut ImageStore) -> Markup {
        html! {
            details open {
                summary {
                    (self.label)
                }
                (self.illustrations.render_html(img_store))
            }
        }
    }
}

#[derive(Debug)]
pub enum ChangeIllustrations {
    PortraitSingle(Vec<(String, Portrait)>),
    PortraitModification(Vec<(String, Portrait, Portrait)>), //old, new
    SpriteSingle(Vec<(String, SpriteSheet)>),
    SpriteModification(Vec<(String, SpriteSheet, SpriteSheet)>), //old, news
}

impl ChangeIllustrations {
    pub fn render_html(&self, image_store: &mut ImageStore) -> Markup {
        match self {
            Self::PortraitSingle(portraits) => {
                html! {
                    div class="contentcontainer" {
                        @for portrait in portraits {
                            div class="contentiner" {
                                span { (portrait.0) }
                                br;
                                @let portrait_path = image_store.add_image(portrait.1.scale(), "todo".into()); //TODO:
                                (embed_image(&portrait_path))
                            }
                        }
                    }
                }
            }
            Self::PortraitModification(portraits) => html! {
                div class="contentcontainer" {
                    @for portrait in portraits {
                        div class="contentiner" {
                            span { (portrait.0) }
                            br;
                            @let old_portrait_path = image_store.add_image(portrait.1.scale(), "todo".into()); //TODO:
                            (embed_image(&old_portrait_path))
                            br;
                            @let new_portrait_path = image_store.add_image(portrait.1.scale(), "todo".into()); //TODO:
                            (embed_image(&new_portrait_path))
                        }
                    }
                }
            },
            Self::SpriteSingle(sprites) => html! {
                div class="contentcontainer" {
                    @for sprite in sprites {
                        div class="contentiner" {
                            span { (sprite.0) }
                            br;
                            @let sprite_path = image_store.add_spritesheet(&sprite.1.scale(4), "todo"); //TODO:
                            (embed_image(&sprite_path))
                        }
                    }
                }
            },
            Self::SpriteModification(sprites) => html! {
                div class="contentcontainer" {
                    @for (name, old_sprite, new_sprite) in sprites {
                        div class="contentiner" {
                            span { (name) }
                            br;
                            @let merged_spritesheet = old_sprite.side_by_side(&new_sprite);
                            @let merged_spritesheet_path = image_store.add_spritesheet(&merged_spritesheet.scale(4), "todo"); //TODO:
                            (embed_image(&merged_spritesheet_path))
                        }
                    }
                }
            },
        }
    }
}