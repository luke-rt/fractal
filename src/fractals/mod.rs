use image::{
	Rgba,
};
use imageproc::drawing::{
	draw_polygon,
};

use crate::util;

mod koch;
mod mandelbrot;
mod sierpinski;

pub enum Fractal {
	Mandelbrot,
	Sierpinski,
	Koch,
}

pub fn render(buf: &mut util::RgbaBuffer, fractal: &Fractal, width: u32, height: u32) -> Result<(), image::ImageError> {
	match fractal {
		Fractal::Mandelbrot => mandelbrot::render(buf, width, height),
		Fractal::Sierpinski => sierpinski::render(buf, width, height),
		Fractal::Koch => koch::render(buf, width, height),
	}
}
