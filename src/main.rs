use ray_tracing_in_one_weekend::color;

const WIDTH: u32 = 256;
const HEIGHT: u32 = 256;
const COLOR_DEPTH: u32 = 255;

fn main() {
    color::write_p3_header(WIDTH, HEIGHT, COLOR_DEPTH);

    for j in 0..HEIGHT {
        eprintln!("Scan lines remaining: {}", HEIGHT - j);
        for i in 0..WIDTH {
            let color = color::Color::new(j as f64 / (HEIGHT - 1) as f64, i as f64 / (WIDTH - 1) as f64, 0.0);
            color::write_p3_color(color, COLOR_DEPTH);
        }
    }

    eprintln!("Finished rendering.")
}
