use image::{
	ImageBuffer,
	Rgba,
};
use rayon::prelude::*;

use crate::fractals;

pub fn write_to_img(filename: &str, dimensions: (u32, u32)) -> Result<(), image::ImageError> {
	let mut imgbuf = ImageBuffer::new(dimensions.0, dimensions.1);

	for (x, y, pixel) in imgbuf.enumerate_pixels_mut() {
        *pixel = fractals::render(fractals::Fractal::Mandelbrot, x, y);
    }

    imgbuf.save(filename)?;

	Ok(())
}
