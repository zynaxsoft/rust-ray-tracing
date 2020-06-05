use std::io::stdout;

use raytrace::vec3::{Color, Point3, unit_vector};
use raytrace::ray::Ray;
use raytrace::util::{write_color, INFINITY, random_number};
use raytrace::hittable::{HitRecord, HittableList};
use raytrace::sphere::Sphere;
use raytrace::camera::Camera;


fn ray_color(r: &Ray, world: &HittableList) -> Color {
    let mut rec = HitRecord::new();
    if world.hit(r, 0.0, INFINITY, &mut rec) {
        return 0.5 * (rec.normal + Color::new(1.0, 1.0, 1.0))
    }
    let unit_direction = unit_vector(r.direction);
    let t = 0.5 * (unit_direction.y + 1.0);
    return
        (1.0 - t) * Color::new(1.0, 1.0, 1.0)
        + t * Color::new(0.5, 0.7, 1.0)
}


fn main() {
    let stdout = stdout();
    let mut stdout = stdout.lock();

    let aspect_ratio = 16.0 / 9.0;
    let image_width: i32 = 384;
    let image_height: i32 = (image_width as f32 / aspect_ratio) as i32;
    let samples_per_pixel: i32 = 5;

    // PPM header
    println!("P3\n{} {}\n255", image_width, image_height);

    let mut world = HittableList::new();
    world.add(Sphere::new(Point3::new(0.0, 0.0, -1.0), 0.5));
    world.add(Sphere::new(Point3::new(0.0, -100.5, -1.0), 100.0));

    let cam = Camera::new();

    for j in (0..image_height).rev() {
        eprint!("\r                           ");
        eprint!("\rScanlines remaining: {}", j);
        for i in 0..image_width {
            let mut pixel_color = Color::new(0.0, 0.0, 0.0);
            for _ in 0..samples_per_pixel {
                let u = (i as f32 + random_number()) / (image_width - 1) as f32;
                let v = (j as f32 + random_number()) / (image_height - 1) as f32;
                let r = cam.get_ray(u, v);
                pixel_color += ray_color(&r, &world);
            }
            write_color(&mut stdout, pixel_color, samples_per_pixel);
        }
    }

    eprintln!("\nDone!");
}
