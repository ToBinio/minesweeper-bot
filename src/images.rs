use std::path::Path;

use image::io::Reader;
use image::{DynamicImage, GenericImageView, SubImage};

pub fn find_sub(img: &DynamicImage, sub: &DynamicImage) -> Option<(u32, u32)> {
    for x in 0..img.width() - sub.width() {
        for y in 0..img.height() - sub.height() {
            let view = img.view(x, y, sub.width(), sub.height());

            if !compare_image(sub, &view) {
                continue;
            }

            for sub_x in 0..sub.width() {
                for sub_y in 0..sub.width() {
                    let sub_rgb = view.get_pixel(sub_x, sub_y).0;
                    let smile_rgb = sub.get_pixel(sub_x, sub_y).0;

                    if compare_color(sub_rgb, smile_rgb) > 3 {}
                }
            }

            return Some((x, y));
        }
    }

    None
}

pub fn open<P: AsRef<Path>>(path: P) -> DynamicImage {
    Reader::open(path).unwrap().decode().unwrap()
}

pub fn compare_image(img: &DynamicImage, sub: &SubImage<&DynamicImage>) -> bool {
    for sub_x in 0..img.width() {
        for sub_y in 0..img.width() {
            let sub_rgb = sub.get_pixel(sub_x, sub_y).0;
            let smile_rgb = img.get_pixel(sub_x, sub_y).0;

            if compare_color(sub_rgb, smile_rgb) > 3 {
                return false;
            }
        }
    }

    true
}

pub fn compare_color(color_a: [u8; 4], color_b: [u8; 4]) -> i32 {
    (color_a[0] as i32 - color_b[0] as i32).pow(2)
        + (color_a[1] as i32 - color_b[1] as i32).pow(2)
        + (color_a[2] as i32 - color_b[2] as i32).pow(2)
}
