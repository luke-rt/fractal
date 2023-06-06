#![forbid(unsafe_code)]
#![warn(warnings)]
#![warn(clippy::all, clippy::pedantic)]

mod img;
mod fractals;

fn main() {
	img::write_to_img("examples/test.png", (256, 256))
		.expect("ERROR: Failed to write to image");

}
