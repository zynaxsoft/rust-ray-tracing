use crate::ray::Ray;
use crate::vec3::{Vec3, Point3, unit_vector, cross};


pub struct Camera {
    pub origin: Point3,
    pub lower_left_corner: Point3,
    pub horizontal: Vec3,
    pub vertical: Vec3,
}

impl Camera {
    pub fn new(lookfrom: Point3, lookat: Point3, vup: Vec3,
               vfov: f32, aspect_ratio: f32) -> Camera {
        let theta = crate::util::deg2rad(vfov);
        let h = (theta / 2.0).tan();
        let viewport_height = 2.0 * h;
        let viewport_width = aspect_ratio * viewport_height;

        let w = unit_vector(lookfrom - lookat);
        let u = unit_vector(cross(vup, w));
        let v = cross(w, u);

        let origin = lookfrom;
        let horizontal = viewport_width * u;
        let vertical = viewport_height * v;
        let lower_left_corner = origin
            - horizontal/2.0
            - vertical/2.0
            - w;
        Camera {
            origin,
            lower_left_corner,
            horizontal,
            vertical,
        }
    }

    pub fn get_ray(&self, u: f32, v:f32) -> Ray {
        let direction = self.lower_left_corner
            + self.horizontal*u
            + self.vertical*v
            - self.origin;
        Ray::new(self.origin, direction)
    }
}
