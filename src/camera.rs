use crate::{
    color::*,
    random::sample_unit_disk_uniform,
    ray::Ray,
    renderable_list::RenderableList,
    vector3::Vector3
};
use rand::Rng;
use std::{
    cell::RefCell, 
    f64,
    rc::Rc
};

#[derive(Clone, Debug, PartialEq)]
pub struct Camera<R: Rng> {
    aspect_ratio: f64,
    look_from: Vector3,
    look_at: Vector3,
    vup: Vector3,
    color_depth: u32,
    decoding_gamma: f64,
    hfov_rad: f64,
    image_width: u32,
    t_min: f64,
    t_max: f64,
    max_depth: u32,
    samples_per_pixel: u32,
    focus_distance: f64,
    defocus_angle: f64,
    rng: Rc<RefCell<R>>
}

impl<R: Rng> Camera<R> {
    pub fn new(
        aspect_ratio: f64,
        look_from: Vector3,
        look_at: Vector3,
        vup: Vector3,
        color_depth: u32,
        decoding_gamma: f64,
        hfov_rad: f64,
        image_width: u32,
        t_min: f64,
        t_max: f64,
        max_depth: u32,
        samples_per_pixel: u32,
        focus_distance: f64,
        defocus_angle: f64,
        rng: Rc<RefCell<R>>
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
            defocus_angle,
            rng
        }
    }

    pub fn render(&mut self, scene: &RenderableList) {
        // Set additional image and camera parameters.
        // Ensure that image_height is at least 1.
        let image_height = if self.aspect_ratio > self.image_width as f64 { 1 } else { (self.image_width as f64 / self.aspect_ratio) as u32 };

        let viewport_width = 2.0 * self.focus_distance * f64::tan(self.hfov_rad / 2.0);
        let viewport_height = viewport_width / (self.image_width as f64 / image_height as f64);

        // Form an orthonormal basis describing the orientation of the camera.
        let w = (self.look_at - self.look_from).normalize();
        let u = Vector3::cross(self.vup, w).normalize();
        let v = w.cross(u);

        let viewport_u = viewport_width * u;
        let viewport_v = -viewport_height * v;
        let viewport_top_left = self.look_from + self.focus_distance * w - (viewport_u + viewport_v) / 2.0;

        let viewport_delta_u = viewport_u / (self.image_width as f64);
        let viewport_delta_v = viewport_v / (image_height as f64);
        let first_pixel_center = viewport_top_left + (viewport_delta_u + viewport_delta_v) / 2.0;

        // Miscellaneous parameters.
        // Radius of the disk used for anti-aliasing.
        let encoding_gamma = self.decoding_gamma.recip();
        let anti_aliasing_disk_r = f64::max(viewport_delta_u.norm(), viewport_delta_v.norm());
        let defocus_radius = self.focus_distance * f64::tan(self.defocus_angle / 2.0);

        // Render.
        write_p3_header(self.image_width, image_height, self.color_depth);

        for j in 0..image_height {
            eprintln!("Scan lines remaining: {}", image_height - j);
            for i in 0..self.image_width {
                let mut acc_color = Vector3::from([0.0; 3]);
                let pixel_center = first_pixel_center + (i as f64) * viewport_delta_u + (j as f64) * viewport_delta_v;
                for _ in 0..self.samples_per_pixel {
                    let anti_aliasing_disk_sample = self.sample_unit_disk_uniform();
                    let defocus_disk_sample = self.sample_unit_disk_uniform();
                    let ray_origin_offset = defocus_radius * (defocus_disk_sample.x() * u + defocus_disk_sample.y() * v);
                    let ray_direction_offset = anti_aliasing_disk_r * Vector3::new(anti_aliasing_disk_sample.x(), 0.0, anti_aliasing_disk_sample.y());
                    let ray_origin = self.look_from + ray_origin_offset;
                    let mut ray = Ray::new(ray_origin, pixel_center + ray_direction_offset - ray_origin);
                    let mut ray_attenuation = Vector3::new(1.0, 1.0, 1.0);
                    let mut ray_color = Vector3::from([0.0; 3]);
                    let mut depth = 0;
                    while depth < self.max_depth {
                        if let Some(intersection) = scene.intersect(ray, self.t_min, self.t_max) {
                            let object = scene.get(intersection.index);
                            ray_attenuation = Vector3::multiply_components(ray_attenuation, object.attenuation(ray, intersection.t));
                            ray = object.scatter(ray, intersection.t);
                        } else {
                            let t: f64 = (ray.direction.normalize().z() + 1.0) / 2.0;
                            ray_color = Vector3::multiply_components(ray_attenuation, lerp(Vector3::from([1.0; 3]), Vector3::new(0.5, 0.7, 1.0), t));
                            break;
                        }
                        depth += 1;
                    }
                    if depth > self.max_depth {
                        ray_color = Vector3::from([0.0; 3]);
                    }
                    acc_color += ray_color
                }
                write_p3_color(acc_color / self.samples_per_pixel as f64, self.color_depth, encoding_gamma);
            }
        }

        eprintln!("Finished rendering.")
    }

    fn sample_unit_disk_uniform(&self) -> Vector3 {
        let rng_ref = &mut self.rng.borrow_mut();
        sample_unit_disk_uniform(rng_ref)
    }
}

pub fn vfov_to_hfov(vfov_rad: f64, aspect_ratio: f64) -> f64 {
    2.0 * f64::atan(aspect_ratio * f64::tan(vfov_rad / 2.0))
}