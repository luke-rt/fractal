use crate::util;

mod mandelbrot;
mod sierpinski;

pub enum Fractal {
	Mandelbrot,
	Sierpinski,
}

pub fn render(buf: &mut util::RgbaBuffer, fractal: &Fractal, width: u32, height: u32) {
	match fractal {
		Fractal::Mandelbrot => mandelbrot::render(buf, width, height),
		Fractal::Sierpinski => sierpinski::render(buf, width, height),
	}
}
