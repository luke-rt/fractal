use image::ImageBuffer;
use rayon::prelude::*;

use crate::fractals;

pub fn write_to_img(filename: &str, dimensions: (u32, u32), fractal: &fractals::Fractal) -> Result<(), image::ImageError> {
	let mut imgbuf = ImageBuffer::new(dimensions.0, dimensions.1);

	fractals::render(&mut imgbuf, fractal, dimensions.0, dimensions.1)?;

	imgbuf.save(filename)?;

	Ok(())
}
