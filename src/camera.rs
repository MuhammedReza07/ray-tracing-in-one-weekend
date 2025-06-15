use crate::{
    color::*,
    random::sample_unit_disk_uniform,
    ray::Ray,
    renderable_list::RenderableList,
    vector3::Vector3
};
use rand::{
    self, 
    Rng,
    SeedableRng,
};
use std::{
    f64,
    sync::{
        Arc,
        Mutex
    },
    thread
};

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Camera {
    aspect_ratio: f64,
    look_from: Vector3,
    look_at: Vector3,
    vup: Vector3,
    color_depth: usize,
    decoding_gamma: f64,
    hfov_rad: f64,
    image_width: usize,
    t_min: f64,
    t_max: f64,
    max_depth: usize,
    samples_per_pixel: usize,
    focus_distance: f64,
    defocus_angle_rad: f64,
}

impl Camera {
    pub fn new(
        aspect_ratio: f64,
        look_from: Vector3,
        look_at: Vector3,
        vup: Vector3,
        color_depth: usize,
        decoding_gamma: f64,
        hfov_rad: f64,
        image_width: usize,
        t_min: f64,
        t_max: f64,
        max_depth: usize,
        samples_per_pixel: usize,
        focus_distance: f64,
        defocus_angle_rad: f64,
    ) -> Self {
        Self {
            aspect_ratio,
            look_from,
            look_at,
            vup,
            color_depth,
            decoding_gamma,
            hfov_rad,
            image_width,
            t_min,
            t_max,
            max_depth,
            samples_per_pixel,
            focus_distance,     // In this case, focus distance = focal length = distance from look_from to image plane.
            defocus_angle_rad,
        }
    }

    pub fn render<R: Rng + ?Sized>(&self, rng: &mut R, scene: &RenderableList<R>) -> Image {
        // Set additional image and camera parameters.
        // Ensure that image_height is at least 1.
        let image_height = if self.aspect_ratio > self.image_width as f64 { 1 } else { (self.image_width as f64 / self.aspect_ratio) as usize };

        let viewport_width = 2.0 * self.focus_distance * f64::tan(self.hfov_rad / 2.0);
        let viewport_height = viewport_width / (self.image_width as f64 / image_height as f64);

        // Form an orthonormal basis describing the orientation of the camera.
        let w = (self.look_at - self.look_from).normalize();
        let u = self.vup.cross(w).normalize();
        let v = w.cross(u);

        let viewport_u = viewport_width * u;
        let viewport_v = -viewport_height * v;
        let viewport_top_left = self.look_from + self.focus_distance * w - (viewport_u + viewport_v) / 2.0;

        let viewport_delta_u = viewport_u / (self.image_width as f64);
        let viewport_delta_v = viewport_v / (image_height as f64);
        let viewport_00 = viewport_top_left + (viewport_delta_u + viewport_delta_v) / 2.0;

        // Miscellaneous parameters.
        // Radius of the disk used for anti-aliasing.
        let anti_aliasing_radius = f64::max(viewport_delta_u.norm(), viewport_delta_v.norm());
        let defocus_radius = self.focus_distance * f64::tan(self.defocus_angle_rad / 2.0);

        // Render.
        let mut image = Image::new(self.image_width, image_height, self.color_depth, self.decoding_gamma.recip());

        for i in 0..image_height {
            eprintln!("Scan lines remaining: {}", image_height - i);
            for j in 0..self.image_width {
                let mut acc_color = Vector3::from([0.0; 3]);
                for _ in 0..self.samples_per_pixel {
                    let viewport_ij = viewport_00 + (j as f64) * viewport_delta_u + (i as f64) * viewport_delta_v;
                    let anti_aliasing_disk_sample = sample_unit_disk_uniform(rng);
                    let defocus_disk_sample = sample_unit_disk_uniform(rng);
                    let ray_origin_offset = defocus_radius * (defocus_disk_sample.x() * u + defocus_disk_sample.y() * v);
                    let ray_direction_offset = anti_aliasing_radius * Vector3::new(anti_aliasing_disk_sample.x(), 0.0, anti_aliasing_disk_sample.y());
                    let ray_origin = self.look_from + ray_origin_offset;
                    let mut ray = Ray::new(ray_origin, viewport_ij + ray_direction_offset - ray_origin);
                    let mut ray_attenuation = Vector3::new(1.0, 1.0, 1.0);
                    let mut ray_color = Vector3::from([0.0; 3]);
                    let mut depth = 0;
                    while depth < self.max_depth {
                        if let Some(intersection) = scene.intersect(ray, self.t_min, self.t_max) {
                            let object = scene.get(intersection.index);
                            ray_attenuation *= object.attenuation(rng, ray, intersection.t);
                            ray = match object.scatter(rng, ray, intersection.t) {
                                Some(r) => r,
                                _ => {
                                    ray_color = Vector3::from([0.0; 3]);
                                    break;
                                }
                            };
                        } else {
                            let t: f64 = (ray.direction.normalize().z() + 1.0) / 2.0;
                            ray_color = ray_attenuation * lerp(Vector3::from([1.0; 3]), Vector3::new(0.5, 0.7, 1.0), t);
                            break;
                        }
                        depth += 1;
                    }
                    if depth > self.max_depth {
                        ray_color = Vector3::from([0.0; 3]);
                    }
                    acc_color += ray_color
                }
                image.set_pixel(acc_color / self.samples_per_pixel as f64, i, j);
            }
        }

        eprintln!("Finished rendering.");
        image
    }

    /// Each thread has its own RNG initialised using `SeedableRng::from_os_rng()`.
    pub fn render_concurrent<R: Rng + SeedableRng + ?Sized + 'static>(
        self, 
        scene: Arc<RenderableList<R>>, 
        thread_count: usize
    ) -> Image {
        // Set additional image and camera parameters.
        // Ensure that image_height is at least 1.
        let image_height = if self.aspect_ratio > self.image_width as f64 { 1 } else { (self.image_width as f64 / self.aspect_ratio) as usize };

        let viewport_width = 2.0 * self.focus_distance * f64::tan(self.hfov_rad / 2.0);
        let viewport_height = viewport_width / (self.image_width as f64 / image_height as f64);

        // Form an orthonormal basis describing the orientation of the camera.
        let w = (self.look_at - self.look_from).normalize();
        let u = self.vup.cross(w).normalize();
        let v = w.cross(u);

        let viewport_u = viewport_width * u;
        let viewport_v = -viewport_height * v;
        let viewport_top_left = self.look_from + self.focus_distance * w - (viewport_u + viewport_v) / 2.0;

        let viewport_delta_u = viewport_u / (self.image_width as f64);
        let viewport_delta_v = viewport_v / (image_height as f64);
        let viewport_00 = viewport_top_left + (viewport_delta_u + viewport_delta_v) / 2.0;

        // Miscellaneous parameters.
        // Radius of the disk used for anti-aliasing.
        let anti_aliasing_radius = f64::max(viewport_delta_u.norm(), viewport_delta_v.norm());
        let defocus_radius = self.focus_distance * f64::tan(self.defocus_angle_rad / 2.0);

        // Render.
        let image = Arc::new(Mutex::new(Image::new(self.image_width, image_height, self.color_depth, self.decoding_gamma.recip())));
        let mut handles = Vec::new();

        for t in 0..thread_count {
            let image = image.clone();
            let scene = scene.clone();
            let handle = thread::spawn(
                move || {
                    let mut rng = R::from_os_rng();
                    let mut i = t;
                    while i < image_height {
                        let mut scan_line = Vec::<Vector3>::new();
                        scan_line.reserve(self.image_width);
                        for j in 0..self.image_width {
                            let mut acc_color = Vector3::from([0.0; 3]);
                            for _ in 0..self.samples_per_pixel {
                                let viewport_ij = viewport_00 + (j as f64) * viewport_delta_u + (i as f64) * viewport_delta_v;
                                let anti_aliasing_disk_sample = sample_unit_disk_uniform(&mut rng);
                                let defocus_disk_sample = sample_unit_disk_uniform(&mut rng);
                                let ray_origin_offset = defocus_radius * (defocus_disk_sample.x() * u + defocus_disk_sample.y() * v);
                                let ray_direction_offset = anti_aliasing_radius * Vector3::new(anti_aliasing_disk_sample.x(), 0.0, anti_aliasing_disk_sample.y());
                                let ray_origin = self.look_from + ray_origin_offset;
                                let mut ray = Ray::new(ray_origin, viewport_ij + ray_direction_offset - ray_origin);
                                let mut ray_attenuation = Vector3::new(1.0, 1.0, 1.0);
                                let mut ray_color = Vector3::from([0.0; 3]);
                                let mut depth = 0;
                                while depth < self.max_depth {
                                    if let Some(intersection) = scene.intersect(ray, self.t_min, self.t_max) {
                                        let object = scene.get(intersection.index);
                                        ray_attenuation *= object.attenuation(&mut rng, ray, intersection.t);
                                        ray = match object.scatter(&mut rng, ray, intersection.t) {
                                            Some(r) => r,
                                            _ => {
                                                ray_color = Vector3::from([0.0; 3]);
                                                break;
                                            }
                                        };
                                    } else {
                                        let t: f64 = (ray.direction.normalize().z() + 1.0) / 2.0;
                                        ray_color = ray_attenuation * lerp(Vector3::from([1.0; 3]), Vector3::new(0.5, 0.7, 1.0), t);
                                        break;
                                    }
                                    depth += 1;
                                }
                                if depth > self.max_depth {
                                    ray_color = Vector3::from([0.0; 3]);
                                }
                                acc_color += ray_color
                            }
                            scan_line.push(acc_color / self.samples_per_pixel as f64);
                        }
                        let mut img = image.lock().unwrap();
                        (*img).set_row(scan_line, i);
                        i += thread_count;
                    }
                }
            );
            handles.push(handle);
        }

        for h in handles {
            h.join().unwrap();
        }

        // All threads have a single reference to image, which should be dropped at this point.
        // Thus, unless the mutex becomes poisoned, this should never panic.
        Arc::into_inner(image).unwrap().into_inner().unwrap()
    }
}

pub fn vfov_to_hfov(vfov_rad: f64, aspect_ratio: f64) -> f64 {
    2.0 * f64::atan(aspect_ratio * f64::tan(vfov_rad / 2.0))
}