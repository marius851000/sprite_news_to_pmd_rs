use image::{
    imageops::{resize, FilterType},
    GenericImage, ImageBuffer, Rgb,
};

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

enum LineContent {
    Text(String),
    PortraitList(Vec<PortraitImage>),
}

impl LineContent {
    fn get_width(&self) -> usize {
        match self {
            LineContent::Text(_) => 100, //TODO: more exact value
            LineContent::PortraitList(p) => p.len() * RESOLUTION_X * INT_SCALE_FACTOR,
        }
    }

    fn get_height(&self) -> usize {
        match self {
            LineContent::Text(_) => 20, //TODO: more exact value
            LineContent::PortraitList(_) => RESOLUTION_Y * INT_SCALE_FACTOR,
        }
    }

    fn write_content(&self, pos_y: u32, buffer: &mut ImageBuffer<Rgb<u8>, Vec<u8>>) {
        match self {
            LineContent::Text(_) => (), //TODO:
            LineContent::PortraitList(p) => {
                for (count, portrait) in p.iter().enumerate() {
                    let x = count * RESOLUTION_Y * INT_SCALE_FACTOR;
                    let scaled = resize(
                        portrait,
                        (RESOLUTION_X * INT_SCALE_FACTOR) as u32,
                        (RESOLUTION_Y * INT_SCALE_FACTOR) as u32,
                        FilterType::Nearest,
                    );
                    buffer.copy_from(&scaled, x as u32, pos_y).unwrap();
                }
            }
        }
    }
}

pub fn present_portrait_picture(
    change: PortraitPicturePresentation,
) -> ImageBuffer<Rgb<u8>, Vec<u8>> {
    let mut lines = Vec::new();
    if change.additions.len() != 0 {
        lines.push(LineContent::Text("added".into()));
        for portrait_line in change.additions.chunks(MAX_PORTRAIT_BY_LINE) {
            lines.push(LineContent::PortraitList(
                portrait_line.iter().cloned().collect::<Vec<_>>(),
            ))
        }
    };

    if !change.deletions.is_empty() {
        lines.push(LineContent::Text("deleted".into()));
        for portrait_line in change.deletions.chunks(MAX_PORTRAIT_BY_LINE) {
            lines.push(LineContent::PortraitList(
                portrait_line.iter().cloned().collect::<Vec<_>>(),
            ));
        }
    };

    if !change.modifications.is_empty() {
        lines.push(LineContent::Text("changed".into()));
        for (old, new) in &change.modifications {
            lines.push(LineContent::PortraitList(vec![old.clone(), new.clone()]));
        }
    }

    let width = lines.iter().map(|x| x.get_width()).max().unwrap_or(0);
    let height: usize = lines.iter().map(|x| x.get_height()).sum();

    let mut result_image = ImageBuffer::new(width as u32, height as u32);

    let mut image_y_position = 0;
    for line in lines.iter() {
        line.write_content(image_y_position, &mut result_image);
        image_y_position += line.get_height() as u32;
    }

    result_image
}
