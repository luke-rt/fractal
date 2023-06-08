use image::{
	Rgba,
};

use crate::util;

pub enum Fractal {
	Mandelbrot,
	Sierpinski,
	Custom,
}

pub fn render(buf: &mut util::RgbaBuffer, fractal: &Fractal, width: u32, height: u32) -> Result<(), image::ImageError> {
	match fractal {
		Fractal::Mandelbrot => render_mandelbrot(buf, width, height),
		Fractal::Sierpinski => render_sierpinski(buf, width, height),
		Fractal::Custom => render_custom(buf, width, height),
	}
}

fn render_mandelbrot(buf: &mut util::RgbaBuffer, width: u32, height: u32) -> Result<(), image::ImageError> {
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

fn render_sierpinski(buf: &mut util::RgbaBuffer, width: u32, height: u32) -> Result<(), image::ImageError> {
	Ok(())
}

fn render_custom(buf: &mut util::RgbaBuffer, width: u32, height: u32) -> Result<(), image::ImageError> {
	for (x, y, pixel) in buf.enumerate_pixels_mut() {
		let cx = (y as f64) * (3.0 / width as f64) - 1.5;
		let cy = (x as f64) * (3.0 / height as f64) - 1.5;

		let c = num::Complex::new(cx, cy);
		let mut z = num::Complex::new(cx, cx);

		// escape time function
		let mut i = 0;
		while i < util::LIMIT && z.norm_sqr() <= 4.0 {
			z = z * z + c;
			i += 1;
		}

		let r = i as u8;
		let g = i as u8;
		let b = i as u8;
		let a = 255 as u8;

		*pixel = Rgba([r, g, b, a]);
	}

	Ok(())
}
