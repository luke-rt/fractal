#![forbid(unsafe_code)]
#![warn(warnings)]
#![warn(clippy::all, clippy::pedantic)]
#[allow(clippy::cast_possible_truncation)]

mod fractals;
mod img;
mod util;

fn main() {
	println!("Enter desired width and height in pixels (only enter ONE value): ");
	let mut line: String = String::new();
	std::io::stdin()
		.read_line(&mut line)
		.expect("Failed to read line");
	let pixels = line
		.trim()
		.parse::<u32>()
		.expect("FAILED TO READ INT FROM INPUT. TERMINATED");

	img::write_to("examples/mandelbrot.png", (pixels, pixels), &fractals::Fractal::Mandelbrot).expect("ERROR: Failed to write to image");
	img::write_to("examples/sierpinski.png", (pixels, pixels), &fractals::Fractal::Sierpinski).expect("ERROR: Failed to write to image");
}
