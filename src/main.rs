const WIDTH: u32 = 256;
const HEIGHT: u32 = 256;
const COLOR_DEPTH: u32 = 255;

fn main() {
    // PPM P3 header.
    println!("P3\n{} {}\n{}", WIDTH, HEIGHT, COLOR_DEPTH);

    for j in 0..HEIGHT {
        eprintln!("Scan lines remaining: {}", HEIGHT - j);
        for i in 0..WIDTH {
            let r = j as f32 / (HEIGHT - 1) as f32;
            let g = i as f32 / (WIDTH - 1) as f32;
            let b: f32 = 0.0;

            let ir = (255.999 * r) as u32;
            let ig = (255.999 * g) as u32;
            let ib = (255.999 * b) as u32;

            println!("{} {} {}", ir, ig, ib);
        }
    }

    eprintln!("Finished rendering.")
}
