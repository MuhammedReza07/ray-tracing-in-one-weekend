use crate::{
    color::*,
    ray::Ray,
    renderable_list::RenderableList,
    vector3::Vector3
};
use std::f64;

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Camera {
    aspect_ratio: f64,
    camera_center: Vector3,
    color_depth: u32,
    focal_length: f64,
    image_width: u32,
    viewport_width: f64,
    t_min: f64,
    t_max: f64
}

impl Camera {
    pub fn new(
        aspect_ratio: f64,
        camera_center: Vector3,
        color_depth: u32,
        focal_length: f64,
        image_width: u32,
        viewport_width: f64,
        t_min: f64,
        t_max: f64
    ) -> Self {
        Self {
            aspect_ratio,
            camera_center,
            color_depth,
            focal_length,
            image_width,
            viewport_width,
            t_min,
            t_max
        }
    }

    pub fn render(&self, scene: &RenderableList) {
        // Set additional image and camera parameters.
        // Ensure that image_height is at least 1.
        let image_height = if self.aspect_ratio > self.image_width as f64 { 1 } else { (self.image_width as f64 / self.aspect_ratio) as u32 };
        let viewport_height = self.viewport_width / (self.image_width as f64 / image_height as f64);

        let camera_center = Vector3::new(0.0, 0.0, 0.0);
        let viewport_u = Vector3::new(self.viewport_width, 0.0, 0.0);
        let viewport_v = Vector3::new(0.0, 0.0, -viewport_height);
        let viewport_offset = Vector3::new(0.0, self.focal_length, 0.0);
        let viewport_top_left = camera_center + viewport_offset - (viewport_u + viewport_v) / 2.0;

        let viewport_delta_u = viewport_u / (self.image_width as f64);
        let viewport_delta_v = viewport_v / (image_height as f64);
        let first_pixel_center = viewport_top_left + (viewport_delta_u + viewport_delta_v) / 2.0;

        // Render.
        write_p3_header(self.image_width, image_height, self.color_depth);

        for j in 0..image_height {
            eprintln!("Scan lines remaining: {}", image_height - j);
            for i in 0..self.image_width {
                let pixel_center = first_pixel_center + (i as f64) * viewport_delta_u + (j as f64) * viewport_delta_v;
                let ray = Ray::new(camera_center, pixel_center - camera_center);
                let color: Color;
                
                if let Some(intersection) = scene.intersect(&ray, self.t_min, self.t_max) {
                    let p = ray.at(intersection.t);
                    let object = scene.get(intersection.index);
                    let n = object.normal(p);
                    color = (Vector3::from([n.x(), n.z(), n.y()]) + Vector3::from([1.0; 3])) / 2.0;
                } else {
                    let t = (ray.direction().normalize().z() + 1.0) / 2.0;
                    color = lerp(&Color::new(1.0, 1.0, 1.0), &Color::new(0.5, 0.7, 1.0), t);
                }
                
                write_p3_color(&color, self.color_depth);
            }
        }

        eprintln!("Finished rendering.")
    }
}