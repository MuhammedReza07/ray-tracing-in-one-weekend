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
    vector3::Vector3
};
use std::{
    cell::RefCell, 
    f64,
    rc::Rc
};
use rand_pcg::Pcg64Mcg;

// Set RNG parameters.
const RNG_SEED: u128 = 0x323030372d30382d33314d696b753339;

// Set image and camera parameters.
const ASPECT_RATIO: f64 = 16.0 / 9.0;
const COLOR_DEPTH: u32 = 255;
const DECODING_GAMMA: f64 = 2.2;
const VFOV_DEG: f64 = 20.0;
const HFOV_DEG: f64 = 90.0;
const IMAGE_WIDTH: u32 = 1200;
const T_MIN: f64 = 0.001;
const T_MAX: f64 = f64::INFINITY;
const MAX_DEPTH: u32 = 64;
// const SAMPLES_PER_PIXEL: u32 = 512;
const SAMPLES_PER_PIXEL: u32 = 16;
const FOCUS_DISTANCE: f64 = 10.0;
const DEFOCUS_ANGLE_DEG: f64 = 0.6;
const MAX_FUZZING_ITERATIONS: u32 = 4;

fn main() {
    // RNG.
    let rng = Rc::new(RefCell::new(Pcg64Mcg::new(RNG_SEED)));

    // Camera.
    let hfov_rad = vfov_to_hfov(VFOV_DEG.to_radians(), ASPECT_RATIO);
    let defocus_angle_rad = DEFOCUS_ANGLE_DEG.to_radians();
    let mut camera = Camera::new(
        ASPECT_RATIO, 
        Vector3::new(13.0, 3.0, 2.0),
        Vector3::new(0.0, 0.0, 0.0),
        Vector3::new(0.0, 0.0, 1.0),
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
        rng.clone()
    );

    // Materials.
    // let no_material = Rc::new(materials::None);
    // let material_ground = Rc::new(Lambertian::new(Vector3::new(0.8, 0.8, 0.0), rng.clone()));
    // let material_center = Rc::new(Lambertian::new(Vector3::new(0.1, 0.2, 0.5), rng.clone()));
    // let material_left = Rc::new(Dielectric::new(Vector3::new(0.0,0.0, 0.0), 1.5, rng.clone()));
    // let material_bubble = Rc::new(Dielectric::new(Vector3::new(0.0, 0.0, 0.0), 1.0 / 1.5, rng.clone()));
    // let material_right = Rc::new(Specular::new(Vector3::new(0.8, 0.6, 0.2)));
    // let material_front = Rc::new(Dielectric::new(Vector3::new(0.25, 2.0, 2.0), 1.5, rng.clone()));
    // let material_back = Rc::new(FuzzySpecular::new(Vector3::new(0.8, 0.8, 0.8), 0.3, rng.clone()));
    let material_ground = Rc::new(Lambertian::new(Vector3::new(0.5, 0.5, 0.5), rng.clone()));
    let material_glass = Rc::new(Dielectric::new(Vector3::new(0.0, 0.0, 0.0), 1.5, rng.clone()));
    let material_metal = Rc::new(FuzzySpecular::new(Vector3::new(0.7, 0.6, 0.5), 0.3, MAX_FUZZING_ITERATIONS, rng.clone()));
    let material_diffuse_brown = Rc::new(Lambertian::new(Vector3::new(0.4, 0.2, 0.1), rng.clone()));
    
    // Scene.
    let mut scene = RenderableList::new();
    // scene.push(Box::new(Sphere::new(Vector3::new(0.0, 0.0, 0.0), 100.0, no_material.clone())));
    // scene.push(Box::new(Sphere::new(Vector3::new(0.0, -1.0, -100.5), 100.0, material_ground.clone())));
    // scene.push(Box::new(Sphere::new(Vector3::new(0.0, -1.2, 0.0), 0.5, material_center.clone())));
    // scene.push(Box::new(Sphere::new(Vector3::new(-1.0, -1.0, 0.0), 0.5, material_left.clone())));
    // scene.push(Box::new(Sphere::new(Vector3::new(-1.0, -1.0, 0.0), 0.4, material_bubble.clone())));
    // scene.push(Box::new(Sphere::new(Vector3::new(1.0, -1.0, 0.0), 0.5, material_right.clone())));
    // scene.push(Box::new(Sphere::new(Vector3::new(0.0, -0.20, 0.0), 0.5, material_front.clone())));
    // scene.push(Box::new(Sphere::new(Vector3::new(0.0, -2.30, 0.0), 0.5, material_back.clone())));
    scene.push(Box::new(Sphere::new(Vector3::new(0.0, 0.0, -1000.0), 1000.0, material_ground.clone())));
    scene.push(Box::new(Sphere::new(Vector3::new(0.0, 0.0, 1.0), 1.0, material_glass.clone())));
    scene.push(Box::new(Sphere::new(Vector3::new(-4.0, 0.0, 1.0), 1.0, material_diffuse_brown.clone())));
    scene.push(Box::new(Sphere::new(Vector3::new(4.0, 0.0, 1.0), 1.0, material_metal.clone())));

    let mut i = -11.0;
    while i < 11.0 {
        let mut j = -11.0;
        while j < 11.0 {
            let rng_ref = &mut rng.borrow_mut();
            let (material_selector, x_offset, y_offset): (f64, f64, f64) = rng_ref.random();
            let center = Vector3::new(i + 0.9 * x_offset, j + 0.9 * y_offset, 0.2);

            if (center - Vector3::new(4.0, 0.0, 0.2)).norm() > 0.9 {
                // Diffuse (Lambertian).
                if material_selector < 0.8 {
                    let (r, g, b): (f64, f64, f64) = rng_ref.random();
                    let material = Rc::new(Lambertian::new(Vector3::new(r, g, b), rng.clone()));
                    scene.push(Box::new(Sphere::new(center, 0.2, material)));
                }
                // (Fuzzy) Specular.
                else if material_selector < 0.95 {
                    let attenuation = Vector3::new(rng_ref.random_range(0.5..1.0), rng_ref.random_range(0.5..1.0), rng_ref.random_range(0.5..1.0));
                    let fuzzing_radius = rng_ref.random_range(0.0..0.5);
                    let material = Rc::new(FuzzySpecular::new(attenuation, fuzzing_radius, MAX_FUZZING_ITERATIONS, rng.clone()));
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

    // Render.
    camera.render(&scene);
}
