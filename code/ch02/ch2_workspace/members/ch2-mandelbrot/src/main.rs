use num::complex::Complex; // <1> Import the `Complex` number type from the `num` crate and its `complex` sub-module

/// This function converts between the output space (a grid of rows and columns) and a range that surrounds 
/// the Mandelbrot set (a continuous region near `(0,0)`).
/// 
/// If a value has not "escaped" before reaching the maximum number of iterations, it is considered to be within the Mandelbrot set.
/// Four parameters specify the space we're searching for to look for members of the set.
/// 
/// `width`, `height` - these two parameters represent the size of the output in "pixels"
fn calculate_mandelbrot(
    max_iters: usize, 
    x_min: f64, 
    x_max: f64, 
    y_min: f64, 
    y_max: f64, 
    width: usize, 
    height: usize, 
) -> Vec<Vec<usize>> {
    let mut all_rows: Vec<Vec<usize>> = Vec::with_capacity(width); 
    for img_y in 0..height {

        let mut row: Vec<usize> = Vec::with_capacity(height);
        for img_x in 0..width {
            let cx = x_min + (x_max - x_min) * (img_x as f64 / width as f64); // <8> Calculate the proportion of the space we have covered in our output and covert that to points within the search space
            let cy = y_min + (y_max - y_min) * (img_y as f64 / height as f64); // <8>
            let escaped_at = mandelbrot_at_point(cx, cy, max_iters); // <9> `cx` and `cy` are the real and imaginary parts of a complex number
            row.push(escaped_at);
        }
        all_rows.push(row);
    }
    all_rows
}

/// This function is called at every "pixel", e.g. for every row and column that's printed to stdout
fn mandelbrot_at_point(cx: f64, cy: f64, max_iters: usize) -> usize {
    let mut z = Complex { re: 0.0, im: 0.0 }; // <11> Initialize a complex number at the origin. That is, with real (`re`) and imaginary (`im`) parts at 0.0.
    let c = Complex::new(cx, cy); // <12> Initialize a complex number from the coordinates provided as function arguments.

    for i in 0..=max_iters {
        if z.norm() > 2.0 {
            // <13> Check the "escape condition". `z.norm()` calculates the distance from the origin (0, 0). It is the absolute value of a complex number.
            return i;
        }
        z = z * z + c; // <14> Repeatedly mutate `z` to check whether `c` lies within the Mandelbrot set
    }
    return max_iters; // <15> `i` is no longer in scope, so we'll fall back to `max_iters`
}

fn render_mandelbrot(escape_vals: Vec<Vec<usize>>) {
    for row in escape_vals {
        let mut line = String::with_capacity(row.len());
        for column in row {
            let val = match column {
                0..=2 => ' ',
                3..=5 => '.',
                6..=10 => '•',
                11..=30 => '*',
                31..=100 => '+',
                101..=200 => 'x',
                201..=400 => '$',
                401..=700 => '#',
                _ => '%',
            };

            line.push(val);
        }
        println!("{}", line);
    }
}

fn main() {
    let mandelbrot = calculate_mandelbrot(1000, -2.0, 1.0, -1.0, 1.0, 80, 24);

    render_mandelbrot(mandelbrot);
}
