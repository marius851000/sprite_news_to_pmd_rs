use image::{
    imageops::{resize, FilterType},
    DynamicImage, RgbaImage,
};

const RESOLUTION_X: usize = 40;
const RESOLUTION_Y: usize = 40;
const INT_SCALE_FACTOR: usize = 4;

#[derive(Debug)]
pub struct Portrait(pub RgbaImage);

impl Portrait {
    pub fn scale(&self) -> DynamicImage {
        //TODO: more configurable scale factor
        DynamicImage::ImageRgba8(resize(
            &self.0,
            (RESOLUTION_X * INT_SCALE_FACTOR) as u32,
            (RESOLUTION_Y * INT_SCALE_FACTOR) as u32,
            FilterType::Nearest,
        ))
    }
}
