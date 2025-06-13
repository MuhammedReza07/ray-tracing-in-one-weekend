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
const VFOV_RAD: f64 = f64::consts::PI * (1.0 / 9.0);
const HFOV_RAD: f64 = f64::consts::FRAC_PI_2;
const IMAGE_WIDTH: u32 = 800;
const T_MIN: f64 = 0.001;
const T_MAX: f64 = f64::INFINITY;
const MAX_DEPTH: u32 = 64;
const SAMPLES_PER_PIXEL: u32 = 64;
const FOCUS_DISTANCE: f64 = 3.4;
const DEFOCUS_ANGLE: f64 = f64::consts::PI * (1.0 / 18.0);

fn main() {
    // RNG.
    let rng = Rc::new(RefCell::new(Pcg64Mcg::new(RNG_SEED)));

    // Camera.
    let hfov_rad = vfov_to_hfov(VFOV_RAD, ASPECT_RATIO);
    let mut camera = Camera::new(
        ASPECT_RATIO, 
        Vector3::new(-2.0, 1.0, 2.0),
        Vector3::new(0.0, -1.0, 0.0),
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
        DEFOCUS_ANGLE,
        rng.clone()
    );

    // Materials.
    let no_material = Rc::new(materials::None);
    let material_ground = Rc::new(Lambertian::new(Vector3::new(0.8, 0.8, 0.0), rng.clone()));
    let material_center = Rc::new(Lambertian::new(Vector3::new(0.1, 0.2, 0.5), rng.clone()));
    let material_left = Rc::new(Dielectric::new(Vector3::new(0.0,0.0, 0.0), 1.5, rng.clone()));
    let material_bubble = Rc::new(Dielectric::new(Vector3::new(0.0, 0.0, 0.0), 1.0 / 1.5, rng.clone()));
    let material_right = Rc::new(Specular::new(Vector3::new(0.8, 0.6, 0.2)));
    // let material_front = Rc::new(Dielectric::new(Vector3::new(0.25, 2.0, 2.0), 1.5, rng.clone()));
    // let material_back = Rc::new(FuzzySpecular::new(Vector3::new(0.8, 0.8, 0.8), 0.3, rng.clone()));

    // Scene.
    let mut scene = RenderableList::new();
    scene.push(Box::new(Sphere::new(Vector3::new(0.0, 0.0, 0.0), 100.0, no_material.clone())));
    scene.push(Box::new(Sphere::new(Vector3::new(0.0, -1.0, -100.5), 100.0, material_ground.clone())));
    scene.push(Box::new(Sphere::new(Vector3::new(0.0, -1.2, 0.0), 0.5, material_center.clone())));
    scene.push(Box::new(Sphere::new(Vector3::new(-1.0, -1.0, 0.0), 0.5, material_left.clone())));
    scene.push(Box::new(Sphere::new(Vector3::new(-1.0, -1.0, 0.0), 0.4, material_bubble.clone())));
    scene.push(Box::new(Sphere::new(Vector3::new(1.0, -1.0, 0.0), 0.5, material_right.clone())));
    // scene.push(Box::new(Sphere::new(Vector3::new(0.0, -0.20, 0.0), 0.5, material_front.clone())));
    // scene.push(Box::new(Sphere::new(Vector3::new(0.0, -2.30, 0.0), 0.5, material_back.clone())));

    // Render.
    camera.render(&mut scene);
}
