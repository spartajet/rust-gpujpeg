use std::{error::Error, fs::File, io::Write, time::Instant};

use opencv::core::MatTraitConst;
use rust_gpujpeg::*;

fn main() -> Result<(), Box<dyn Error>> {
    let cu_stream = std::ptr::null_mut::<CUstream_st>();

    let param = unsafe { gpujpeg_default_parameters() };
    let param_ptr = &param as *const gpujpeg_parameters as *mut gpujpeg_parameters;

    let mut param_image = unsafe { gpujpeg_default_image_parameters() };
    param_image.width = 34270;
    param_image.height = 22931;
    param_image.color_space = gpujpeg_color_space_GPUJPEG_YCBCR_JPEG; // GPUJPEG_COLOR_SPACE_YCBCR;
    param_image.pixel_format = gpujpeg_pixel_format_GPUJPEG_U8;

    let param_image_ptr = &mut param_image as *mut gpujpeg_image_parameters;

    let mut encoder_input = gpujpeg_encoder_input {
        type_: 0,
        image: std::ptr::null_mut(),
        texture: std::ptr::null_mut(),
    };
    let encoder = unsafe { gpujpeg_encoder_create(cu_stream) };
    assert!(!encoder.is_null(), "the encoder is null");

    let result = unsafe { gpujpeg_init_device(0, GPUJPEG_INIT_DEV_VERBOSE as i32) };
    assert_eq!(result, 0, "Failed to initialize GPUJPEG device");

    let encoder_input_ptr =
        &mut encoder_input as *const gpujpeg_encoder_input as *mut gpujpeg_encoder_input;
    let _size = unsafe { gpujpeg_image_calculate_size(param_image_ptr) };

    let origin_mat = opencv::imgcodecs::imread(
        "test_image/Plushies-Summoner.jpg",
        opencv::imgcodecs::IMREAD_GRAYSCALE,
    )?;
    let start = Instant::now();
    // let mut blank_buffer = Vec::<u8>::with_capacity(size);
    // let blank_buffer_ptr = blank_buffer.as_mut_ptr();
    let blank_buffer_ptr = origin_mat.data() as *mut u8;
    unsafe { gpujpeg_encoder_input_set_image(encoder_input_ptr, blank_buffer_ptr) };
    let out_ptr: *mut u8 = std::ptr::null_mut();
    let out_ptr_ptr = &out_ptr as *const *mut u8 as *mut *mut u8;
    let mut out_size: usize = 0;
    let out_size_ptr = &mut out_size as *mut usize;

    let result = unsafe {
        gpujpeg_encoder_encode(
            encoder,
            param_ptr,
            param_image_ptr,
            encoder_input_ptr,
            out_ptr_ptr,
            out_size_ptr,
        )
    };
    let elapsed = start.elapsed();
    println!("执行耗时: {:4}ms", elapsed.as_millis());
    assert_eq!(result, 0, "Encoding failed with error code: {}", result);
    let buf = unsafe { std::slice::from_raw_parts(out_ptr, out_size) };
    // let mut buf_copy = vec![0u8; out_size];
    // buf_copy.copy_from_slice(buf);
    println!("out size:{}", out_size);
    assert!(!buf.is_empty(), "Encoded buffer is empty");
    let mut out_file = File::create("test_image/large-2.jpeg")?;
    out_file.write_all(buf)?;
    out_file.flush()?;
    let elapsed = start.elapsed();
    println!("执行耗时: {:4}ms", elapsed.as_millis());
    let result = unsafe { gpujpeg_encoder_destroy(encoder) };
    assert_eq!(result, 0, "Failed to destroy the encoder");
    println!("Encoding completed successfully, output saved to test_image/large-2.jpeg");
    Ok(())
}
