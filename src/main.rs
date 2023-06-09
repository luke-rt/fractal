#![forbid(unsafe_code)]
#![warn(warnings)]
#![warn(clippy::all, clippy::pedantic)]
#[allow(clippy::cast_possible_truncation)]

mod fractals;
mod img;
mod util;

fn main() {
	let width = 1024;
	let height = 1024;
	img::write_to("examples/mandelbrot.png", (width, height), &fractals::Fractal::Mandelbrot).expect("ERROR: Failed to write to image");
	img::write_to("examples/sierpinski.png", (width, height), &fractals::Fractal::Sierpinski).expect("ERROR: Failed to write to image");
}
