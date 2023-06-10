# Final Project Name: Fractal Generator in Rust
## Names: Luke Tong and Wa Fan

We decided to implement this project because we thought fractals looked really cool, especially the Mandelbrot set. We were also inspired by the Recursive Graphics assignment earlier in the year where we plotted a Sierpinski triangle in Java earlier in the year.

Steps to run:
- Method 1: Download the executable from [releases page](https://github.com/luketio/fractal/releases/tag/v1.0)
- Method 2: If that doesn't work, first [Install the Rust Programming Language](https://www.rust-lang.org/tools/install), then clone the repository and run `cargo run`

Data types: Point, Arrays, Complex Number, ImgBuffer, Pixel

ADT Methods:
- `img::write_to` Opens a new image and puts it in ImgBuffer which allows its pixels to be manipulated
- `mandelbrot::render` Renders a Mandelbrot set. Using complex numbers, it graphs the graph of f(z) = z * z + c where z and c are complex numbers and c is a constant. It determines escapetime, essentially how detailed to draw it, by the number of pixels
- `sierpinski::render` Renders a Sierpinski Triangle recursively. Determines n, the number of times it recurses, by the number of pixels.

Problems we encountered:
- It was difficult to find resources to write this code in Rust, since Rust is a bit of a less used language than Java or C++

Other comments:
- Rust is my favorite language, highly recommend
