use image::{
	ImageBuffer,
	Rgba,
};

pub const LIMIT: u32 = 255;

pub type RgbaBuffer = ImageBuffer<Rgba<u8>, Vec<u8>>;
