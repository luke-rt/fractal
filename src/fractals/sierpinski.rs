use image::{
	Rgba,
};

use imageproc::point::Point;
use imageproc::drawing::{
	draw_polygon_mut,
};

use crate::util;

const SQRT_3_HALFS: f64 = 0.86602540378; // L + no const fn

/// Rendering a Chaos Game fractal, the Sierpinski Triangle
pub fn render(buf: &mut util::RgbaBuffer, width: u32, height: u32) -> Result<(), image::ImageError> {
	for (_x, _y, pixel) in buf.enumerate_pixels_mut() {
		*pixel = Rgba([255, 255, 255, 255]);
	}
	let n = width / 128;
	sierpinski(buf, n, (width / 2) as i32, height as i32, (width / 2) as i32)?;

	Ok(())
}

fn sierpinski(buf: &mut util::RgbaBuffer, n: u32, x: i32, y: i32, s: i32) -> Result<(), image::ImageError> {
	if n == 0 { return Ok(()); }

	draw_rtriangle_mut(buf, (x, y), s)?;

	sierpinski(buf, n-1, x - s/2, y, s/2)?;
	sierpinski(buf, n-1, x + s/2, y, s/2)?;
    sierpinski(buf, n-1, x, y - (SQRT_3_HALFS * s as f64) as i32, s/2)?;

	Ok(())
}

fn draw_rtriangle_mut(buf: &mut util::RgbaBuffer, bottom: (i32, i32), side_len: i32) -> Result<(), image::ImageError> {
	let black = Rgba([0, 0, 0, 255]);
	let tmp =  (SQRT_3_HALFS * (side_len as f64)) as i32;

	let p1 = Point::new(bottom.0, bottom.1);
	let p2 = Point::new(bottom.0 + (side_len / 2), bottom.1 - tmp);
	let p3 = Point::new(bottom.0 - (side_len / 2), bottom.1 - tmp);

	if p1 == p2 || p1 == p3 || p2 == p3 {
		return Ok(())
	}

	draw_polygon_mut::<util::RgbaBuffer>(
		buf,
		&[
			p1,
			p2,
			p3
		],
		black,
	);

	Ok(())
}
