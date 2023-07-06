// mod dangling_pointer;
// mod extract_csv;
// mod nba_stars;
mod mandelbrot;

// use dangling_pointer::dangling_pointer;
// use extract_csv::extract_csv;

fn main() {
    let mandelbrot = mandelbrot::calculate_mandelbrot(1000, -2.0, 1.0, -1.0, 1.0, 100, 24);
    mandelbrot::render(mandelbrot);
}
