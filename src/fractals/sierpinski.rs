use image::Rgba;

use imageproc::drawing::draw_polygon_mut;
use imageproc::point::Point;

use crate::util;

const SQRT_3_HALFS: f64 = 0.866_025_403_78; // L + no const fn

/// Rendering a Chaos Game fractal, the Sierpinski Triangle
pub fn render(buf: &mut util::RgbaBuffer, width: u32, height: u32) {
	for (_x, _y, pixel) in buf.enumerate_pixels_mut() {
		*pixel = Rgba([255, 255, 255, 255]);
	}
	let n = width / 128;
	sierpinski(buf, n, (width / 2) as i32, height as i32, (width / 2) as i32);
}

/// sierpinski recursive function. Draws triangle, then runs 3 more times
fn sierpinski(buf: &mut util::RgbaBuffer, n: u32, x: i32, y: i32, s: i32) {
	if n == 0 {
		return;
	}

	draw_rtriangle_mut(buf, (x, y), s);

	sierpinski(buf, n - 1, x - s / 2, y, s / 2);
	sierpinski(buf, n - 1, x + s / 2, y, s / 2);
	sierpinski(buf, n - 1, x, y - (SQRT_3_HALFS * f64::from(s)) as i32, s / 2);
}

/// draws a reverse triangle given the length of the median
fn draw_rtriangle_mut(buf: &mut util::RgbaBuffer, bottom: (i32, i32), side_len: i32) {
	let black = Rgba([0, 0, 0, 255]);
	let tmp = (SQRT_3_HALFS * f64::from(side_len)) as i32;

	let p1 = Point::new(bottom.0, bottom.1);
	let p2 = Point::new(bottom.0 + (side_len / 2), bottom.1 - tmp);
	let p3 = Point::new(bottom.0 - (side_len / 2), bottom.1 - tmp);

	if p1 == p2 || p1 == p3 || p2 == p3 {
		return;
	}

	draw_polygon_mut::<util::RgbaBuffer>(buf, &[p1, p2, p3], black);
}
