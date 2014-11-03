use libc::{c_int, c_uchar};

#[link(name = "stb-image-resize", kind = "static")]
extern {
    pub fn stbir_resize_uint8(input_pixels: *const c_uchar,
                              input_w: c_int,
                              input_h: c_int,
                              input_stride_in_bytes: c_int,
                              output_pixels: *mut c_uchar,
                              output_w: c_int,
                              output_h: c_int,
                              output_stride_in_bytes: c_int,
                              num_channels: c_int) -> c_int;
}
