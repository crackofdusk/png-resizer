extern crate libc;
extern crate stb_image;
extern crate stb_image_resize;
extern crate stb_image_write;

use std::os;
use libc::{c_int, c_void};
use stb_image_resize::stb_image_resize as image_resize;
use stb_image_write::stb_image_write as image_write;

fn main() {
    let args = os::args();

    if args.len() != 3 {
        println!("Usage: {} INPUT OUTPUT", args[0]);
        return;
    }

    let input = std::path::Path::new(args[1].clone());
    let output = std::path::Path::new(args[2].clone());

    // FIXME: allow passing as command line arguments
    let width_factor = 2f64;
    let height_factor = 2f64;

    resize(&input, width_factor, height_factor, &output);
}

fn resize(input: &Path, width_factor: f64, height_factor: f64, output: &Path) {
    let image = match stb_image::image::load(input) {
        stb_image::image::ImageU8(image) => image,
        _ => panic!("could not load image")
    };

    let new_width = (image.width as f64 * width_factor) as i32;
    let new_height = (image.height as f64 * height_factor) as i32;

    let mut output_pixels = Vec::new();

    // FIXME: don't hardcode this
    let nchannels = 4;

    unsafe {
        image_resize::stbir_resize_uint8(image.data.as_ptr(),
                                         image.width as c_int,
                                         image.height as c_int,
                                         0,
                                         output_pixels.as_mut_ptr(),
                                         new_width as c_int,
                                         new_height as c_int,
                                         0,
                                         nchannels);

        image_write::stbi_write_png(output.as_str().unwrap().as_ptr() as *const i8,
                                    new_width,
                                    new_height,
                                    nchannels,
                                    output_pixels.as_ptr() as *const c_void,
                                    0);
    }
}
