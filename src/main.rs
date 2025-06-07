use ray_tracing_in_one_weekend::{
    camera::Camera,
    renderable_list::RenderableList,
    surfaces::sphere::Sphere, 
    vector3::Vector3
};
use rand_pcg::Pcg64Mcg;

// Set RNG parameters.
const RNG_SEED: u128 = 0x323030372d30382d33314d696b753339;

// Set image and camera parameters.
const ASPECT_RATIO: f64 = 16.0 / 9.0;
const COLOR_DEPTH: u32 = 255;
const FOCAL_LENGTH: f64 = -1.0;
const IMAGE_WIDTH: u32 = 400;
const VIEWPORT_WIDTH: f64 = 1.0;
const T_MIN: f64 = 0.0;
const T_MAX: f64 = f64::INFINITY;
const SAMPLES_PER_PIXEL: u32 = 32;

fn main() {
    // RNG.
    let rng = Pcg64Mcg::new(RNG_SEED);

    // Camera.
    let mut camera = Camera::new(
        ASPECT_RATIO, 
        Vector3::new(0.0, 0.0, 0.0),
        COLOR_DEPTH,
        FOCAL_LENGTH,
        IMAGE_WIDTH,
        VIEWPORT_WIDTH,
        T_MIN,
        T_MAX,
        SAMPLES_PER_PIXEL,
        rng
    );

    // Scene.
    let mut scene = RenderableList::new();
    scene.push(Box::new(Sphere::new(Vector3::new(0.0, -3.0, 0.0), 0.5)));
    scene.push(Box::new(Sphere::new(Vector3::new(0.0, -3.0, -100.5), 100.0)));

    // Render.
    camera.render(&mut scene);
}
