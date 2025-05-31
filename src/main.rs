use ray_tracing_in_one_weekend::{
    color,
    ray,
    vector3::Vector3
};

fn main() {
    // Image.
    const ASPECT_RATIO: f64 = 16.0 / 9.0;
    const COLOR_DEPTH: u32 = 255;
    const IMAGE_WIDTH: u32 = 400;
    // Ensure that IMAGE_HEIGHT is at least 1.
    const IMAGE_HEIGHT: u32 = if ASPECT_RATIO > IMAGE_WIDTH as f64 { 1 } else { (IMAGE_WIDTH as f64 / ASPECT_RATIO) as u32 };

    // Camera.
    const FOCAL_LENGTH: f64 = 1.0;
    const VIEWPORT_WIDTH: f64 = 1.0;
    const VIEWPORT_HEIGHT: f64 = VIEWPORT_WIDTH / (IMAGE_WIDTH as f64 / IMAGE_HEIGHT as f64);

    let camera_center = Vector3::new(0.0, 0.0, 0.0);
    let viewport_u = Vector3::new(VIEWPORT_WIDTH, 0.0, 0.0);
    let viewport_v = Vector3::new(0.0, 0.0, -VIEWPORT_HEIGHT);
    let viewport_offset = Vector3::new(0.0, FOCAL_LENGTH, 0.0);
    let viewport_top_left = camera_center + viewport_offset - (viewport_u + viewport_v) / 2.0;

    let viewport_delta_u = viewport_u / (IMAGE_WIDTH as f64);
    let viewport_delta_v = viewport_v / (IMAGE_HEIGHT as f64);
    let first_pixel_center = viewport_top_left + (viewport_delta_u + viewport_delta_v) / 2.0;

    // Render.
    color::write_p3_header(IMAGE_WIDTH, IMAGE_HEIGHT, COLOR_DEPTH);

    for j in 0..IMAGE_HEIGHT {
        eprintln!("Scan lines remaining: {}", IMAGE_HEIGHT - j);
        for i in 0..IMAGE_WIDTH {
            let pixel_center = first_pixel_center + (i as f64) * viewport_delta_u + (j as f64) * viewport_delta_v;
            let ray = ray::Ray::new(camera_center, pixel_center - camera_center);
            let t = (ray.direction().normalize().z() + 1.0) / 2.0;
            let color = color::lerp(&color::Color::new(1.0, 1.0, 1.0), &color::Color::new(0.5, 0.7, 1.0), t);
            color::write_p3_color(&color, COLOR_DEPTH);
        }
    }
    
    eprintln!("Finished rendering.")
}
