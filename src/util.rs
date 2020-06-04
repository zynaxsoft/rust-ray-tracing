use std::io::Write;
use crate::vec3::Color;


pub fn write_color<T>(stream: &mut T, pixel_color: Color)
where T: Write {
    let ir = (255.999 * pixel_color.x) as i32;
    let ig = (255.999 * pixel_color.y) as i32;
    let ib = (255.999 * pixel_color.z) as i32;
    let res = write!(stream, "{} {} {}\n", ir, ig, ib);
    match res {
        Ok(_) => return,
        Err(_) => return,
    }
}
