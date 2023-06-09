use image::{
	Rgba,
};

use crate::util;

/// Rendering an Escape Time fractal, the Mandelbrot Set
pub fn render(buf: &mut util::RgbaBuffer, width: u32, height: u32) -> Result<(), image::ImageError> {
	for (x, y, pixel) in buf.enumerate_pixels_mut() {

		// determines field/location of view
		// the random constants are to optimize the location of the mandelbrot fractal in the image
		let c = num::Complex::new(
			(x as f64 * 3.0) / (width as f64) - 2.0,
			(y as f64 * 3.0) / (height as f64) - 1.5,
		);
		// "seed"
		let mut z = num::Complex::new(0.0, 0.0);

		// escape time function
		let mut i: u32 = 0;
		while i < util::LIMIT && z.norm_sqr() <= 2.0 {
			z = z * z + c;
			i += 1;
		}
		let intensity = if i == util::LIMIT { 0 } else {(255.0 * (i as f64 / util::LIMIT as f64).sqrt()) as u8};

		// hsv to rgb
		*pixel = Rgba([intensity, intensity, intensity, 255]);
	}

	Ok(())
}
