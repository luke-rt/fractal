use image::{
	Rgba,
};

pub enum Fractal {
	Mandelbrot,
}

pub fn render(fractal: Fractal, x: u32, y: u32) -> Rgba<u8> {
	match fractal {
		Fractal::Mandelbrot => render_mandelbrot(x, y)
	}
}

fn render_mandelbrot(x: u32, y: u32) -> Rgba<u8> {
	let r = (0.3 * x as f32) as u8;
	let g = 0 as u8;
    let b = (0.3 * y as f32) as u8;
	let a = x as u8;

	Rgba([r, g, b, a])
}
