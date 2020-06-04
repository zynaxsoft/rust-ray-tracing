use std::io::stdout;

use raytrace::vec3::{
    Color,
    Point3,
    Vec3,
    dot,
    unit_vector,
};
use raytrace::ray::Ray;
use raytrace::util::write_color;


fn hit_sphere(center: Point3, radius: f32, r: &Ray) -> bool {
    let oc = r.origin - center;
    let a = dot(r.direction, r.direction);
    let b = 2.0 * dot(oc, r.direction);
    let c = dot(oc, oc) - radius*radius;
    let discriminant = b.powi(2) - 4.0*a*c;
    discriminant > 0.0
}


fn ray_color(r: &Ray) -> Color {
    if hit_sphere(Point3::new(0.0, 0.0, -1.0), 0.5, r) {
        return Color::new(1.0, 0.0, 0.0)
    }
    let unit_direction = unit_vector(r.direction);
    let t = 0.5 * (unit_direction.y + 1.0);
    return
        (1.0 - t) * Color::new(1.0, 1.0, 1.0)
        + t * Color::new(0.5, 0.7, 1.0)
}


fn main() {
    let aspect_ratio = 16.0 / 9.0;
    let image_width: i32 = 384;
    let image_height: i32 = (image_width as f32 / aspect_ratio) as i32;

    // PPM header
    println!("P3\n{} {}\n255", image_width, image_height);

    let viewport_height = 2.0;
    let viewport_width = aspect_ratio * viewport_height;
    let focal_length = 1.0;

    let origin = Point3::new(0.0, 0.0, 0.0);
    let horizontal = Vec3::new(viewport_width, 0.0, 0.0);
    let vertical = Vec3::new(0.0, viewport_height, 0.0);
    let lower_left_corner = origin
        - horizontal/2.0
        - vertical/2.0
        - Vec3::new(0.0, 0.0, focal_length);

    let stdout = stdout();
    let mut stdout = stdout.lock();

    for j in (0..image_height).rev() {
        eprint!("\rScanlines remaining: {}", j);
        for i in 0..image_width {
            let u = i as f32 / (image_width - 1) as f32;
            let v = j as f32 / (image_height -1 ) as f32;
            let direction = lower_left_corner
                + horizontal*u
                + vertical*v
                - origin;
            let r = Ray::new(origin, direction);
            let pixel_color = ray_color(&r);
            write_color(&mut stdout, pixel_color);
        }
    }

    eprintln!("\nDone!");
}
