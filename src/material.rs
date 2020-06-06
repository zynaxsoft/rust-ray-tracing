use crate::vec3::{Vec3, Color, unit_vector, dot};
use crate::ray::Ray;
use crate::util::random_number;
use crate::hittable::HitRecord;


pub trait Material {
    fn scatter(&self, r_in: &Ray, rec: &HitRecord, attenuation: &mut Color, scattered: &mut Ray) -> bool;
}

pub struct Lambertian {
    pub albedo: Color,
}

impl Lambertian {
    pub fn new(albedo: Color) -> Lambertian {
        Lambertian {
            albedo
        }
    }
}

impl Material for Lambertian {
    fn scatter(&self, _r_in: &Ray, rec: &HitRecord, attenuation: &mut Color, scattered: &mut Ray) -> bool {
        let scatter_direction = rec.normal + Vec3::random_unit_vector();
        *scattered = Ray::new(rec.p, scatter_direction);
        *attenuation = self.albedo;
        true
    }
}

pub struct Metal {
    pub albedo: Color,
    pub fuzz: f32,
}

impl Metal {
    pub fn new(albedo: Color, fuzz: f32) -> Metal {
        Metal {
            albedo,
            fuzz: if fuzz < 1.0 {fuzz} else {1.0},
        }
    }
}

impl Material for Metal {
    fn scatter(&self, r_in: &Ray, rec: &HitRecord, attenuation: &mut Color, scattered: &mut Ray) -> bool {
        let reflected = Vec3::reflect(unit_vector(r_in.direction), rec.normal);
        *scattered = Ray::new(rec.p, reflected + self.fuzz * Vec3::random_in_unit_sphere());
        *attenuation = self.albedo;
        dot(scattered.direction, rec.normal) > 0.0
    }
}


pub struct Dielectric {
    pub ref_idx: f32,
}

impl Dielectric {
    pub fn new(ref_idx: f32) -> Dielectric {
        Dielectric {
            ref_idx,
        }
    }

    fn schlick(cosine: f32, ref_idx: f32) -> f32 {
        let r0 = (1.0 - ref_idx) / (1.0 + ref_idx);
        let r0 = r0.powi(2);
        r0 + (1.0 - r0)*(1.0 - cosine).powi(5)
    }
}

impl Material for Dielectric {
    fn scatter(&self, r_in: &Ray, rec: &HitRecord, attenuation: &mut Color, scattered: &mut Ray) -> bool {
        *attenuation = Color::new(1.0, 1.0, 1.0);
        let etai_over_etat = if rec.front_face {
            1.0 / self.ref_idx
        } else {
            self.ref_idx
        };

        let unit_direction = unit_vector(r_in.direction);
        let cos_theta = 1.0_f32.min(dot(-unit_direction, rec.normal));
        let sin_theta = (1.0 - cos_theta.powi(2)).sqrt();
        if etai_over_etat * sin_theta > 1.0 {
            let reflected = Vec3::reflect(unit_direction, rec.normal);
            *scattered = Ray::new(rec.p, reflected);
            return true
        }

        let reflect_prob = Dielectric::schlick(cos_theta, etai_over_etat);
        if random_number() < reflect_prob {
            let reflected = Vec3::reflect(unit_direction, rec.normal);
            *scattered = Ray::new(rec.p, reflected);
            return true
        }
        let refracted = Vec3::refract(unit_direction, rec.normal, etai_over_etat);
        *scattered = Ray::new(rec.p, refracted);
        true
    }
}
