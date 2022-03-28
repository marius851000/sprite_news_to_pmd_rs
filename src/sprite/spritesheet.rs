use std::io::Write;

use apng::{self, load_dynamic_image};
use image::{
    imageops::{resize, FilterType},
    DynamicImage, GenericImage, GenericImageView, ImageBuffer, Rgba,
};

use crate::{git_change::SpriteSheetContent, sprite::AnimData};

#[derive(Debug, Clone)]
pub struct Frame {
    pub anim: ImageBuffer<Rgba<u8>, Vec<u8>>,
    pub duration: u32,
}

impl Frame {
    pub fn merge_with(&self, other: &Self, duration: u32) -> Self {
        let width = self.anim.width() + other.anim.width();
        let height = self.anim.height().max(other.anim.height());

        let mut new_anim = ImageBuffer::new(width, height);

        new_anim.copy_from(&self.anim, 0, 0).unwrap();
        new_anim
            .copy_from(&other.anim, self.anim.width(), 0)
            .unwrap();
        Frame {
            anim: new_anim,
            duration,
        }
    }

    pub fn scale(&self, factor: u32) -> Self {
        Frame {
            anim: resize(
                &self.anim,
                self.anim.width() * factor,
                self.anim.height() * factor,
                FilterType::Nearest,
            ),
            duration: self.duration,
        }
    }
}

#[derive(Default, Debug, Clone)]
pub struct DirAnim {
    pub frames: Vec<Frame>,
}

impl DirAnim {
    pub fn scale(&self, factor: u32) -> Self {
        Self {
            frames: self.frames.iter().map(|x| x.scale(factor)).collect(),
        }
    }
}

impl DirAnim {
    pub fn duration(&self) -> u32 {
        self.frames.iter().map(|x| x.duration).sum()
    }
}

#[derive(Debug, Clone)]
pub struct SpriteSheet {
    pub dir_anims: Vec<DirAnim>,
    frame_size: (u32, u32),
}

impl SpriteSheet {
    pub fn size(&self) -> &(u32, u32) {
        &self.frame_size
    }

    pub fn new_from_change(change: &SpriteSheetContent) -> Self {
        let (animdata_str, invalid_char) =
            encoding_rs::UTF_8.decode_with_bom_removal(&change.animdata);
        if invalid_char {
            panic!("Invalid character were found while decoding the xml as UTF-8");
        };
        let animdata_global: AnimData = serde_xml_rs::from_str(animdata_str.as_ref()).unwrap();

        let anim_image = image::load_from_memory(&change.anim).unwrap().into_rgba8();

        let animdata = animdata_global
            .anims
            .get_item_by_name_follow_copy(&change.name)
            .unwrap();

        let frame_size = (
            animdata.frame_width.unwrap(),
            animdata.frame_height.unwrap(),
        );
        let frame_number = (
            anim_image.width() / frame_size.0,
            anim_image.height() / frame_size.1,
        );

        let durations = &animdata.durations.as_ref().unwrap().durations;

        let mut result = SpriteSheet {
            dir_anims: Vec::new(),
            frame_size: frame_size.clone(),
        };

        for dir_nb in 0..frame_number.1 {
            let mut anim = DirAnim::default();
            for frame_nb in 0..frame_number.0 {
                let anim_frame = anim_image
                    .view(
                        frame_nb * frame_size.0,
                        dir_nb * frame_size.1,
                        frame_size.0,
                        frame_size.1,
                    )
                    .to_image();
                anim.frames.push(Frame {
                    anim: anim_frame,
                    duration: durations[frame_nb as usize],
                })
            }
            result.dir_anims.push(anim);
        }

        result
    }

    pub fn number_frame(&self) -> u32 {
        self.dir_anims
            .iter()
            .map(|dir_anim| dir_anim.frames.len())
            .sum::<usize>() as u32
    }

    pub fn side_by_side(&self, other: &SpriteSheet) -> Self {
        let new_frame_size = (
            self.frame_size.0 + other.frame_size.0,
            self.frame_size.1.max(other.frame_size.1),
        );
        let mut result = SpriteSheet {
            frame_size: new_frame_size.clone(),
            dir_anims: Vec::new(),
        };

        for (self_dir_anim, other_dir_anim) in self.dir_anims.iter().zip(other.dir_anims.iter()) {
            let mut new_dir_anim = DirAnim::default();
            let max_duration = self_dir_anim.duration().max(other_dir_anim.duration());
            let mut state: [(_, u32, _); 2] = [
                (
                    0,
                    self_dir_anim.frames[0].duration,
                    &self_dir_anim.frames[0],
                ),
                (
                    0,
                    other_dir_anim.frames[0].duration,
                    &other_dir_anim.frames[0],
                ),
            ]; // 1st: the frame number, 2nd: the number of frame before next frame image, 3rd is last remembered object
            let mut old_frame_duration = 0;
            for framecount in 0..max_duration {
                old_frame_duration += 1;
                let mut should_make_new_frame = false;
                match state[0].1.checked_sub(1) {
                    Some(ok) => state[0].1 = ok,
                    None => {
                        should_make_new_frame = true;
                        let potential_new_frame_count = state[0].0 + 1;
                        if let Some(new_frame) = self_dir_anim.frames.get(potential_new_frame_count)
                        {
                            state[0].0 += potential_new_frame_count;
                            state[0].1 = new_frame.duration;
                            state[0].2 = new_frame;
                        } else {
                            state[0].1 = max_duration.wrapping_sub(framecount);
                        }
                    }
                }

                match state[1].1.checked_sub(1) {
                    Some(ok) => state[1].1 = ok,
                    None => {
                        should_make_new_frame = true;
                        let potential_new_frame_count = state[1].0 + 1;
                        if let Some(new_frame) =
                            other_dir_anim.frames.get(potential_new_frame_count)
                        {
                            state[1].0 += potential_new_frame_count;
                            state[1].1 = new_frame.duration;
                            state[1].2 = new_frame;
                        } else {
                            state[1].1 = max_duration.wrapping_sub(framecount);
                        }
                    }
                }

                if should_make_new_frame {
                    let new_merged_frame = state[0].2.merge_with(state[1].2, old_frame_duration);
                    new_dir_anim.frames.push(new_merged_frame);
                    old_frame_duration = 0;
                }
            }
            if old_frame_duration != 0 {
                let new_merged_frame = state[0].2.merge_with(state[1].2, old_frame_duration);
                new_dir_anim.frames.push(new_merged_frame);
            }
            result.dir_anims.push(new_dir_anim);
        }

        result
    }

    pub fn write_apng<F: Write>(&self, writer: &mut F) {
        let config = apng::Config {
            width: self.frame_size.0,
            height: self.frame_size.1,
            num_frames: self.number_frame(),
            num_plays: 0,
            color: png::ColorType::RGBA,
            depth: png::BitDepth::Eight,
            filter: png::FilterType::NoFilter,
        };

        let mut encoder = apng::Encoder::new(writer, config).unwrap();

        for dir_anim in &self.dir_anims {
            for frame in &dir_anim.frames {
                let png_frame = apng::Frame {
                    delay_num: Some(frame.duration as u16),
                    delay_den: Some(30),
                    ..Default::default()
                };
                encoder
                    .write_frame(
                        &load_dynamic_image(DynamicImage::ImageRgba8(frame.anim.clone())).unwrap(),
                        png_frame,
                    )
                    .unwrap();
            }
        }

        encoder.finish_encode().unwrap();
    }

    pub fn scale(&self, factor: u32) -> Self {
        SpriteSheet {
            dir_anims: self
                .dir_anims
                .iter()
                .map(|dir_anim| dir_anim.scale(factor))
                .collect(),
            frame_size: (self.frame_size.0 * factor, self.frame_size.1 * factor),
        }
    }
}
