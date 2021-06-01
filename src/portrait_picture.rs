use image::{
    imageops::{resize, FilterType},
    DynamicImage, GenericImage, ImageBuffer, Rgb, Rgba,
};

use crate::{output::{ChangeIllustration, ChangeIllustrationImage}};

pub type PortraitImage = ImageBuffer<Rgb<u8>, Vec<u8>>;

const RESOLUTION_X: usize = 40;
const RESOLUTION_Y: usize = 40;

const MAX_PORTRAIT_BY_LINE: usize = 6;
const INT_SCALE_FACTOR: usize = 4;

/// Represent change that can be displayed with [`present_portrait_picture`]
pub struct PortraitPicturePresentation {
    /// A portrait has been added (new portrait is stored)
    pub additions: Vec<PortraitImage>,
    /// A portrait has changed (first, old portrait, then, new portrait)
    pub modifications: Vec<(PortraitImage, PortraitImage)>,
    /// A portrait has been deleted (old portrait is stored)
    pub deletions: Vec<PortraitImage>,
}

#[derive(Default)]
struct PortraitCollection(Vec<Vec<PortraitImage>>);

impl PortraitCollection {
    fn get_width(&self) -> usize {
        self.0.iter().map(|e| e.len()).max().unwrap_or(0) * RESOLUTION_X * INT_SCALE_FACTOR
    }

    fn get_height(&self) -> usize {
       self.0.len() * RESOLUTION_Y * INT_SCALE_FACTOR
    }

    fn create_image(&self) -> ImageBuffer<Rgba<u8>, Vec<u8>> {
        let mut image = ImageBuffer::new(self.get_width() as u32, self.get_height() as u32);
        for (line_nb, line) in self.0.iter().enumerate() {
            let y = line_nb * RESOLUTION_Y * INT_SCALE_FACTOR;
            for (portrait_nb, portrait) in line.iter().enumerate() {
                let x = portrait_nb * RESOLUTION_X * INT_SCALE_FACTOR;
                let portrait_rgba = DynamicImage::ImageRgb8(portrait.clone()).into_rgba8();
                let scaled = resize(
                    &portrait_rgba,
                    (RESOLUTION_X * INT_SCALE_FACTOR) as u32,
                    (RESOLUTION_Y * INT_SCALE_FACTOR) as u32,
                    FilterType::Nearest,
                );
                image.copy_from(&scaled, x as u32, y as u32).unwrap();
            }
        }
        image
    }
}

fn illustration_from_list_of_portrait(portraits: &[PortraitImage], title: String, name_tip: String) -> ChangeIllustration {
    let portrait_collection: Vec<Vec<PortraitImage>> = portraits
        .chunks(MAX_PORTRAIT_BY_LINE)
        .map(|portrait_line| portrait_line.to_vec())
        .collect::<Vec<_>>();
    ChangeIllustration {
        filename_tip: name_tip,
        title,
        image: ChangeIllustrationImage::Portraits(PortraitCollection(portrait_collection).create_image()),
    }
}

pub fn present_portrait_picture(change: PortraitPicturePresentation, name_tip: String) -> Vec<ChangeIllustration> {
    let mut illustrations = Vec::new();

    if !change.additions.is_empty() {
        illustrations.push(illustration_from_list_of_portrait(&change.additions, "added".into(), name_tip.clone()));
    };

    if !change.deletions.is_empty() {
        illustrations.push(illustration_from_list_of_portrait(&change.deletions, "deleted".into(), name_tip.clone()));
    };

    if !change.modifications.is_empty() {
        let mut portrait_collection = PortraitCollection::default();

        for (old_portrait, new_portrait) in change.modifications.into_iter() {
            portrait_collection.0.push(vec![
                old_portrait, new_portrait
            ]);
        };

        illustrations.push(ChangeIllustration {
            filename_tip: name_tip.clone(),
            title: "old -> new".into(),
            image: ChangeIllustrationImage::Portraits(portrait_collection.create_image()),
        });
    }

    illustrations
}
