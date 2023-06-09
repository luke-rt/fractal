#![forbid(unsafe_code)]
#![warn(warnings)]
#![warn(clippy::all, clippy::pedantic)]

mod fractals;
mod img;
mod util;

fn main() {
	let width = 1024;
	let height = 1024;
	let fractal = fractals::Fractal::Sierpinski;
	img::write_to_img("examples/sierpinski.png", (width, height), &fractal).expect("ERROR: Failed to write to image");
}
