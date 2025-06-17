use crate::{
    color::*, random::sample_unit_disk_uniform, ray::Ray, renderable_list::RenderableList, vector4::Vector4
};
use rand::{
    self, 
    Rng,
    SeedableRng,
};
use std::{
    iter,
    sync::{Arc, mpsc},
    thread
};

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Camera {
    // Image.
    aspect_ratio: f32,
    image_width: usize,
    image_height: usize,
    color_depth: usize,
    decoding_gamma: f32,
    hfov_rad: f32,
    focus_distance: f32,
    // Orientation.
    look_from: Vector4,
    look_at: Vector4,
    vup: Vector4,
    // Orthonormal basis describing camera orientation.
    u: Vector4,             // Unit vector in a direction orthogonal to v and w (camera right).
    v: Vector4,             // Unit vector denoting the camera's up direction.
    w: Vector4,             // Unit vector in the direction opposite that of look_at - look_from.
    // Viewport.
    viewport_00: Vector4,
    viewport_delta_u: Vector4,
    viewport_delta_v: Vector4,
    // Sampling.
    samples_per_pixel: usize,
    anti_aliasing_disk_radius: f32,
    defocus_disk_radius: f32,
    // Ray intersections.
    max_depth: usize,
    t_min: f32,
    t_max: f32
}

impl Camera {
    pub fn new(
        // Image.
        aspect_ratio: f32,
        image_width: usize,
        color_depth: usize,
        decoding_gamma: f32,
        hfov_rad: f32,
        focus_distance: f32,
        look_from: Vector4,
        look_at: Vector4,
        vup: Vector4,
        samples_per_pixel: usize,
        defocus_angle_rad: f32,
        max_depth: usize,
        t_min: f32,
        t_max: f32
    ) -> Self {
        // Ensure that image_height is at least 1.
        let image_height = if aspect_ratio > image_width as f32 { 1 } else { (image_width as f32 / aspect_ratio) as usize };

        let viewport_width = 2.0 * focus_distance * f32::tan(hfov_rad / 2.0);
        let viewport_height = viewport_width / (image_width as f32 / image_height as f32);

        // Form an orthonormal basis describing the orientation of the camera.
        let w = (look_at - look_from).normalize();
        let u = vup.cross(w).normalize();
        let v = w.cross(u);

        let viewport_u = viewport_width * u;
        let viewport_v = -viewport_height * v;
        let viewport_top_left = look_from + focus_distance * w - (viewport_u + viewport_v) / 2.0;

        let viewport_delta_u = viewport_u / (image_width as f32);
        let viewport_delta_v = viewport_v / (image_height as f32);

        Self {
            aspect_ratio,
            image_width,
            image_height,
            color_depth,
            decoding_gamma,
            hfov_rad,
            focus_distance,
            look_from,
            look_at,
            vup,
            u,
            v,
            w,
            viewport_00: viewport_top_left + (viewport_delta_u + viewport_delta_v) / 2.0,
            viewport_delta_u,
            viewport_delta_v,
            samples_per_pixel,
            anti_aliasing_disk_radius: f32::max(viewport_delta_u.norm(), viewport_delta_v.norm()),
            defocus_disk_radius: focus_distance * f32::tan(defocus_angle_rad / 2.0),
            max_depth,
            t_min,
            t_max
        }
    }

    pub fn render<R: Rng + ?Sized>(&self, rng: &mut R, scene: &RenderableList<R>) -> Image {
        let mut image = Image::new(self.image_width, self.image_height, self.color_depth, self.decoding_gamma.recip());

        for i in 0..self.image_height {
            eprintln!("Scan lines remaining: {}", self.image_height - i);
            for j in 0..self.image_width {
                let mut acc_color = Vector4::new(0.0, 0.0, 0.0, 0.0);
                for _ in 0..self.samples_per_pixel {
                    let ray = self.ray(rng, i, j);
                    acc_color += self.ray_color(
                        rng, 
                        ray, 
                        scene
                    );
                }
                image.set_pixel(acc_color / self.samples_per_pixel as f32, i, j);
            }
        }
        eprintln!("Finished rendering.");

        image
    }

    // TODO: Handle the case where thread_count = 1 or thread_count = 0 (if relevant).
    /// Each thread has its own RNG initialised using `SeedableRng::from_os_rng()`.
    pub fn render_concurrent<R: Rng + SeedableRng + 'static>(
        self, 
        scene: Arc<RenderableList<R>>, 
        thread_count: usize
    ) -> Image {
        let mut image = Image::new(
            self.image_width, 
            self.image_height, 
            self.color_depth, 
            self.decoding_gamma.recip()
        );
        let mut handles = Vec::new();
        let (tx, rx) = mpsc::sync_channel::<(usize, Vec<Vector4>)>(thread_count);

        for t in 0..thread_count {
            let tx = tx.clone();
            let scene = scene.clone();
            let handle = thread::spawn(
                move || {
                    let mut rng = R::from_os_rng();
                    // Initialise scan line buffer (old, with preallocation).
                    // let div = self.image_height / thread_count;
                    // let offset = if self.image_height - thread_count * div >= t + 1 { 1 } else { 0 };
                    // let mut scan_lines: Vec<Vector4> = Vec::with_capacity(self.image_width * (div + offset));
                    let scan_lines: Vec<Vector4> = (t..self.image_height)
                    .step_by(thread_count)
                    .flat_map(|i| iter::repeat_n(i, self.image_width))
                    .zip((0..self.image_width).cycle())
                    .map(|(i, j)| {
                        let mut acc_color = Vector4::new(0.0, 0.0, 0.0, 0.0);
                        for _ in 0..self.samples_per_pixel {
                            let ray = self.ray(&mut rng, i, j);
                            acc_color += self.ray_color(
                                &mut rng, 
                                ray, 
                                &scene
                            );
                        }
                        acc_color / self.samples_per_pixel as f32
                    })
                    .collect();
                    /* let mut i = t;
                    while i < self.image_height {
                        for j in 0..self.image_width {
                            let mut acc_color = Vector4::new(0.0, 0.0, 0.0, 0.0);
                            for _ in 0..self.samples_per_pixel {
                                let ray = self.ray(&mut rng, i, j);
                                acc_color += self.ray_color(
                                    &mut rng, 
                                    ray, 
                                    &scene
                                );
                            }
                            scan_lines.push(acc_color / self.samples_per_pixel as f32);
                        }
                        i += thread_count;
                    } */
                    tx.send((t, scan_lines)).unwrap();
                }
            );
            handles.push(handle);
        }

        for _ in 0..thread_count {
            let (t, scan_lines) = rx.recv().unwrap();
            (t..self.image_height)
            .step_by(thread_count)
            .enumerate()
            .for_each(|(j, i)| image.set_row(&scan_lines[j * self.image_width..(j + 1) * self.image_width], i))
        }

        image
    }

    fn ray<R: Rng + ?Sized>(&self, rng: &mut R, i: usize, j: usize) -> Ray {
        let viewport_ij = self.viewport_00 + (j as f32) * self.viewport_delta_u + (i as f32) * self.viewport_delta_v;
        let anti_aliasing_disk_sample = sample_unit_disk_uniform(rng);
        let defocus_disk_sample = sample_unit_disk_uniform(rng);
        let ray_origin_offset = self.defocus_disk_radius * (defocus_disk_sample.x() * self.u + defocus_disk_sample.y() * self.v);
        let ray_direction_offset = self.anti_aliasing_disk_radius * Vector4::new(anti_aliasing_disk_sample.x(), 0.0, anti_aliasing_disk_sample.y(), 0.0);
        let ray_origin = self.look_from + ray_origin_offset;
        Ray::new(ray_origin, viewport_ij + ray_direction_offset - ray_origin)
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