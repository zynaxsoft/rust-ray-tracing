use std::rc::Rc;

use crate::hittable::{Hittable, HitRecord};
use crate::ray::Ray;
use crate::vec3::{Point3, dot};
use crate::material::Material;


pub struct Sphere {
    pub center: Point3,
    pub radius: f32,
    pub mat_ptr: Rc<dyn Material>,
}

impl Sphere {
    pub fn new(center: Point3, radius: f32, mat_ptr: Rc<dyn Material>) -> Sphere {
        Sphere {
            center,
            radius,
            mat_ptr,
        }
    }
}

impl Hittable for Sphere {
    fn hit(&self, r: &Ray, t_min: f32, t_max: f32, rec: &mut HitRecord) -> bool {
        let oc = r.origin - self.center;
        let a = r.direction.length_squared();
        let half_b = dot(oc, r.direction);
        let c = dot(oc, oc) - self.radius.powi(2);
        let discriminant = half_b.powi(2) - a*c;
        if discriminant > 0.0 {
            let root = discriminant.sqrt();
            let temp = (-half_b - root) / a;
            if temp < t_max && temp > t_min {
                rec.t = temp;
                rec.p = r.at(rec.t);
                let outward_normal = (rec.p - self.center) / self.radius;
                rec.set_face_normal(r, outward_normal);
                rec.mat_ptr = Some(Rc::clone(&self.mat_ptr));
                return true
            }
            let temp = (-half_b + root) / a;
            if temp < t_max && temp > t_min {
                rec.t = temp;
                rec.p = r.at(rec.t);
                let outward_normal = (rec.p - self.center) / self.radius;
                rec.set_face_normal(r, outward_normal);
                rec.mat_ptr = Some(Rc::clone(&self.mat_ptr));
                return true
            }
        }
        false
    }
}
