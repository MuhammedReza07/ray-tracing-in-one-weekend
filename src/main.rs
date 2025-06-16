use rand::Rng;
use ray_tracing_in_one_weekend::{
    camera::{
        Camera,
        vfov_to_hfov
    },
    materials::{
        self,
        dielectric::Dielectric,
        diffuse::Diffuse,
        fuzzy_specular::FuzzySpecular,
        lambertian::Lambertian,
        specular::Specular
    },
    renderable_list::RenderableList,
    surfaces::sphere::Sphere, 
    vector4::Vector4
};
use std::sync::Arc;
use rand_pcg::Pcg64Mcg;

// Set RNG parameters.
const RNG_SEED: u128 = 0x323030372d30382d33314d696b753339;

// Set image and camera parameters.
const ASPECT_RATIO: f32 = 16.0 / 9.0;
const COLOR_DEPTH: usize = 255;
const DECODING_GAMMA: f32 = 2.2;
const VFOV_DEG: f32 = 20.0;
const HFOV_DEG: f32 = 90.0;
const IMAGE_WIDTH: usize = 1200;
// const IMAGE_WIDTH: usize = 800;
const T_MIN: f32 = 0.001;
const T_MAX: f32 = f32::INFINITY;
const MAX_DEPTH: usize = 64;
// const SAMPLES_PER_PIXEL: usize = 512;
const SAMPLES_PER_PIXEL: usize = 16;
const FOCUS_DISTANCE: f32 = 10.0;
const DEFOCUS_ANGLE_DEG: f32 = 0.6;
const MAX_FUZZING_ITERATIONS: usize = 4;

fn main() {
    // RNG.
    let mut rng = Pcg64Mcg::new(RNG_SEED);

    // Camera.
    let hfov_rad = vfov_to_hfov(VFOV_DEG.to_radians(), ASPECT_RATIO);
    let defocus_angle_rad = DEFOCUS_ANGLE_DEG.to_radians();
    let camera = Camera::new(
        ASPECT_RATIO, 
        Vector4::new(13.0, 3.0, 2.0, 0.0),
        Vector4::new(0.0, 0.0, 0.0, 0.0),
        Vector4::new(0.0, 0.0, 1.0, 0.0),
        COLOR_DEPTH,
        DECODING_GAMMA,
        hfov_rad,
        IMAGE_WIDTH,
        T_MIN,
        T_MAX,
        MAX_DEPTH,
        SAMPLES_PER_PIXEL,
        FOCUS_DISTANCE,
        defocus_angle_rad,
    );

    // Materials.
    let material_ground = Arc::new(Lambertian::new(Vector4::new(0.5, 0.5, 0.5, 0.0)));
    let material_glass = Arc::new(Dielectric::new(Vector4::new(0.0, 0.0, 0.0, 0.0), 1.5));
    let material_metal = Arc::new(FuzzySpecular::new(Vector4::new(0.7, 0.6, 0.5, 0.0), 0.3, MAX_FUZZING_ITERATIONS));
    let material_diffuse_brown = Arc::new(Lambertian::new(Vector4::new(0.4, 0.2, 0.1, 0.0)));
    
    // Scene.
    let mut scene = RenderableList::<Pcg64Mcg>::new();
    scene.push(Box::new(Sphere::new(Vector4::new(0.0, 0.0, -1000.0, 0.0), 1000.0, material_ground.clone())));
    scene.push(Box::new(Sphere::new(Vector4::new(0.0, 0.0, 1.0, 0.0), 1.0, material_glass.clone())));
    scene.push(Box::new(Sphere::new(Vector4::new(-4.0, 0.0, 1.0, 0.0), 1.0, material_diffuse_brown.clone())));
    scene.push(Box::new(Sphere::new(Vector4::new(4.0, 0.0, 1.0, 0.0), 1.0, material_metal.clone())));

    let mut i = -11.0;
    while i < 11.0 {
        let mut j = -11.0;
        while j < 11.0 {
            let (material_selector, x_offset, y_offset): (f32, f32, f32) = rng.random();
            let center = Vector4::new(i + 0.9 * x_offset, j + 0.9 * y_offset, 0.2, 0.0);

            if (center - Vector4::new(4.0, 0.0, 0.2, 0.0)).norm() > 0.9 {
                // Diffuse (Lambertian).
                if material_selector < 0.8 {
                    let (r, g, b): (f32, f32, f32) = rng.random();
                    let material = Arc::new(Lambertian::new(Vector4::new(r, g, b, 0.0)));
                    scene.push(Box::new(Sphere::new(center, 0.2, material)));
                }
                // (Fuzzy) Specular.
                else if material_selector < 0.95 {
                    let attenuation = Vector4::new(rng.random_range(0.5..1.0), rng.random_range(0.5..1.0), rng.random_range(0.5..1.0), 0.0);
                    let fuzzing_radius = rng.random_range(0.0..0.5);
                    let material = Arc::new(FuzzySpecular::new(attenuation, fuzzing_radius, MAX_FUZZING_ITERATIONS));
                    scene.push(Box::new(Sphere::new(center, 0.2, material)));
                }
                // Dielectric (clear glass).
                else {
                    scene.push(Box::new(Sphere::new(center, 0.2, material_glass.clone())));
                }
            }

            j += 1.0;
        }

        i += 1.0;
    }

    let scene = Arc::new(scene);

    // let image = camera.render(&mut rng, &scene);
    // image.write_p3_image_stdout();
    let image = camera.render_concurrent(scene.clone(), 4);
    image.write_p3_image_stdout();
}