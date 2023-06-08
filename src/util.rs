use image::{
	ImageBuffer,
	Rgba,
};
use num::Complex;
use palette::{
	IntoColor,
	Srgb,
	Hsv
};

pub const LIMIT: u32 = 255;

pub type RgbaBuffer = ImageBuffer<Rgba<u8>, Vec<u8>>;

pub fn hsv_to_rgba(hue: f64, saturation: f64, value: f64) -> Rgba<u8> {
	let rgb: Srgb = Hsv::new(hue, saturation, value).into_format().into_color();
	Rgba([
		rgb.red as u8,
		rgb.green as u8,
		rgb.blue as u8,
		255,
	])
}
