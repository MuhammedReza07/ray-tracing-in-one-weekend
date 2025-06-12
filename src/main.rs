use ray_tracing_in_one_weekend::{
    camera::Camera,
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
    rc::Rc
};
use rand_pcg::Pcg64Mcg;

// Set RNG parameters.
const RNG_SEED: u128 = 0x323030372d30382d33314d696b753339;

// Set image and camera parameters.
const ASPECT_RATIO: f64 = 16.0 / 9.0;
const COLOR_DEPTH: u32 = 255;
const DECODING_GAMMA: f64 = 2.2;
const FOCAL_LENGTH: f64 = -1.0;
const IMAGE_WIDTH: u32 = 800;
const VIEWPORT_WIDTH: f64 = 1.0;
const T_MIN: f64 = 0.001;
const T_MAX: f64 = f64::INFINITY;
const MAX_DEPTH: u32 = 64;
const SAMPLES_PER_PIXEL: u32 = 64;

fn main() {
    // RNG.
    let rng = Rc::new(RefCell::new(Pcg64Mcg::new(RNG_SEED)));

    // Camera.
    let mut camera = Camera::new(
        ASPECT_RATIO, 
        Vector3::new(0.0, 0.0, 0.0),
        COLOR_DEPTH,
        DECODING_GAMMA,
        FOCAL_LENGTH,
        IMAGE_WIDTH,
        VIEWPORT_WIDTH,
        T_MIN,
        T_MAX,
        MAX_DEPTH,
        SAMPLES_PER_PIXEL,
        rng.clone()
    );

    // Materials.
    let no_material = Rc::new(materials::None);
    let material_ground = Rc::new(Lambertian::new(Vector3::new(0.8, 0.8, 0.0), rng.clone()));
    let material_center = Rc::new(Lambertian::new(Vector3::new(0.1, 0.2, 0.5), rng.clone()));
    let material_left = Rc::new(Dielectric::new(Vector3::new(0.0,0.0, 0.0), 1.5, rng.clone()));
    let material_bubble = Rc::new(Dielectric::new(Vector3::new(0.0, 0.0, 0.0), 1.0 / 1.5, rng.clone()));
    let material_right = Rc::new(FuzzySpecular::new(Vector3::new(0.8, 0.6, 0.2), 1.0, rng.clone()));

    // Scene.
    let mut scene = RenderableList::new();
    scene.push(Box::new(Sphere::new(Vector3::new(0.0, 0.0, 0.0), 100.0, no_material.clone())));
    scene.push(Box::new(Sphere::new(Vector3::new(0.0, -3.0, -100.5), 100.0, material_ground.clone())));
    scene.push(Box::new(Sphere::new(Vector3::new(0.0, -3.2, 0.0), 0.5, material_center.clone())));
    scene.push(Box::new(Sphere::new(Vector3::new(-1.0, -3.0, 0.0), 0.5, material_left.clone())));
    scene.push(Box::new(Sphere::new(Vector3::new(-1.0, -3.0, 0.0), 0.4, material_bubble.clone())));
    scene.push(Box::new(Sphere::new(Vector3::new(1.0, -3.0, 0.0), 0.5, material_right.clone())));

    // Render.
    camera.render(&mut scene);
}
