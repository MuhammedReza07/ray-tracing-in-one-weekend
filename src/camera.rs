use crate::{
    color::*,
    random::sample_unit_disk_uniform,
    ray::Ray,
    renderable_list::RenderableList,
    vector4::Vector4
};
use rand::{
    self, 
    Rng,
    SeedableRng,
};
use std::{
    sync::{Arc, Mutex},
    thread
};

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Camera {
    aspect_ratio: f32,
    look_from: Vector4,
    look_at: Vector4,
    vup: Vector4,
    color_depth: usize,
    decoding_gamma: f32,
    hfov_rad: f32,
    image_width: usize,
    t_min: f32,
    t_max: f32,
    max_depth: usize,
    samples_per_pixel: usize,
    focus_distance: f32,
    defocus_angle_rad: f32,
}

impl Camera {
    pub fn new(
        aspect_ratio: f32,
        look_from: Vector4,
        look_at: Vector4,
        vup: Vector4,
        color_depth: usize,
        decoding_gamma: f32,
        hfov_rad: f32,
        image_width: usize,
        t_min: f32,
        t_max: f32,
        max_depth: usize,
        samples_per_pixel: usize,
        focus_distance: f32,
        defocus_angle_rad: f32,
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
        let image_height = if self.aspect_ratio > self.image_width as f32 { 1 } else { (self.image_width as f32 / self.aspect_ratio) as usize };

        let viewport_width = 2.0 * self.focus_distance * f32::tan(self.hfov_rad / 2.0);
        let viewport_height = viewport_width / (self.image_width as f32 / image_height as f32);

        // Form an orthonormal basis describing the orientation of the camera.
        let w = (self.look_at - self.look_from).normalize();
        let u = self.vup.cross(w).normalize();
        let v = w.cross(u);

        let viewport_u = viewport_width * u;
        let viewport_v = -viewport_height * v;
        let viewport_top_left = self.look_from + self.focus_distance * w - (viewport_u + viewport_v) / 2.0;

        let viewport_delta_u = viewport_u / (self.image_width as f32);
        let viewport_delta_v = viewport_v / (image_height as f32);
        let viewport_00 = viewport_top_left + (viewport_delta_u + viewport_delta_v) / 2.0;

        // Miscellaneous parameters.
        // Radius of the disk used for anti-aliasing.
        let anti_aliasing_radius = f32::max(viewport_delta_u.norm(), viewport_delta_v.norm());
        let defocus_radius = self.focus_distance * f32::tan(self.defocus_angle_rad / 2.0);

        // Render.
        let mut image = Image::new(self.image_width, image_height, self.color_depth, self.decoding_gamma.recip());

        for i in 0..image_height {
            eprintln!("Scan lines remaining: {}", image_height - i);
            for j in 0..self.image_width {
                let mut acc_color = Vector4::new(0.0, 0.0, 0.0, 0.0);
                for _ in 0..self.samples_per_pixel {
                    let viewport_ij = viewport_00 + (j as f32) * viewport_delta_u + (i as f32) * viewport_delta_v;
                    let anti_aliasing_disk_sample = sample_unit_disk_uniform(rng);
                    let defocus_disk_sample = sample_unit_disk_uniform(rng);
                    let ray_origin_offset = defocus_radius * (defocus_disk_sample.x() * u + defocus_disk_sample.y() * v);
                    let ray_direction_offset = anti_aliasing_radius * Vector4::new(anti_aliasing_disk_sample.x(), 0.0, anti_aliasing_disk_sample.y(), 0.0);
                    let ray_origin = self.look_from + ray_origin_offset;
                    acc_color += self.ray_color(
                        rng, 
                        Ray::new(ray_origin, viewport_ij + ray_direction_offset - ray_origin), 
                        scene
                    );
                }
                image.set_pixel(acc_color / self.samples_per_pixel as f32, i, j);
            }
        }

        eprintln!("Finished rendering.");
        image
    }

    /// Each thread has its own RNG initialised using `SeedableRng::from_os_rng()`.
    pub fn render_concurrent<R: Rng + SeedableRng + 'static>(
        self, 
        scene: Arc<RenderableList<R>>, 
        thread_count: usize
    ) -> Image {
        // Set additional image and camera parameters.
        // Ensure that image_height is at least 1.
        let image_height = if self.aspect_ratio > self.image_width as f32 { 1 } else { (self.image_width as f32 / self.aspect_ratio) as usize };

        let viewport_width = 2.0 * self.focus_distance * f32::tan(self.hfov_rad / 2.0);
        let viewport_height = viewport_width / (self.image_width as f32 / image_height as f32);

        // Form an orthonormal basis describing the orientation of the camera.
        let w = (self.look_at - self.look_from).normalize();
        let u = self.vup.cross(w).normalize();
        let v = w.cross(u);

        let viewport_u = viewport_width * u;
        let viewport_v = -viewport_height * v;
        let viewport_top_left = self.look_from + self.focus_distance * w - (viewport_u + viewport_v) / 2.0;

        let viewport_delta_u = viewport_u / (self.image_width as f32);
        let viewport_delta_v = viewport_v / (image_height as f32);
        let viewport_00 = viewport_top_left + (viewport_delta_u + viewport_delta_v) / 2.0;

        // Miscellaneous parameters.
        // Radius of the disk used for anti-aliasing.
        let anti_aliasing_radius = f32::max(viewport_delta_u.norm(), viewport_delta_v.norm());
        let defocus_radius = self.focus_distance * f32::tan(self.defocus_angle_rad / 2.0);

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
                    // Initialise scan line buffer.
                    let zero_vec = Vector4::new(0.0, 0.0, 0.0, 0.0);
                    let mut scan_line = Vec::with_capacity(self.image_width);
                    for _ in 0..self.image_width {
                        scan_line.push(zero_vec);
                    }
                    while i < image_height {
                        for j in 0..self.image_width {
                            let mut acc_color = Vector4::new(0.0, 0.0, 0.0, 0.0);
                            for _ in 0..self.samples_per_pixel {
                                let viewport_ij = viewport_00 + (j as f32) * viewport_delta_u + (i as f32) * viewport_delta_v;
                                let anti_aliasing_disk_sample = sample_unit_disk_uniform(&mut rng);
                                let defocus_disk_sample = sample_unit_disk_uniform(&mut rng);
                                let ray_origin_offset = defocus_radius * (defocus_disk_sample.x() * u + defocus_disk_sample.y() * v);
                                let ray_direction_offset = anti_aliasing_radius * Vector4::new(anti_aliasing_disk_sample.x(), 0.0, anti_aliasing_disk_sample.y(), 0.0);
                                let ray_origin = self.look_from + ray_origin_offset;
                                acc_color += self.ray_color(
                                    &mut rng, 
                                    Ray::new(ray_origin, viewport_ij + ray_direction_offset - ray_origin), 
                                    &scene
                                );
                            }
                            scan_line[j] = acc_color / self.samples_per_pixel as f32;
                        }
                        let mut img = image.lock().unwrap();
                        (*img).set_row(&scan_line, i);
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

    fn ray_color<R: Rng + ?Sized>(&self, rng: &mut R, r: Ray, scene: &RenderableList<R>) -> Vector4 {
        let mut ray = r;
        let mut ray_attenuation = Vector4::new(1.0, 1.0, 1.0, 0.0);
        for _ in 0..self.max_depth {
            if let Some(intersection) = scene.intersect(ray, self.t_min, self.t_max) {
                let object = scene.get(intersection.index);
                ray_attenuation *= object.attenuation(rng, ray, intersection.t);
                if let Some(r) = object.scatter(rng, ray, intersection.t) {
                    ray = r;
                } else {
                    break;
                }
            } else {
                let t = (ray.direction.normalize().z() + 1.0) / 2.0;
                return ray_attenuation * lerp(Vector4::new(1.0, 1.0, 1.0, 0.0), Vector4::new(0.5, 0.7, 1.0, 0.0), t);
            }
        }
        Vector4::new(0.0, 0.0, 0.0, 0.0)
    }
}

pub fn vfov_to_hfov(vfov_rad: f32, aspect_ratio: f32) -> f32 {
    2.0 * f32::atan(aspect_ratio * f32::tan(vfov_rad / 2.0))
}