#![allow(clippy::all)]
#![allow(dead_code)]
#![allow(unused_imports)]
#![allow(unused_variables)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
include!("bindings.rs");

pub struct CUstreamStWarpper(pub *mut CUstream_st);
unsafe impl Send for CUstreamStWarpper {}
unsafe impl Sync for CUstreamStWarpper {}

pub struct GpujpegParametersWarper(pub *mut gpujpeg_parameters);
unsafe impl Send for GpujpegParametersWarper {}
unsafe impl Sync for GpujpegParametersWarper {}

pub struct GpujpegImageParametersWarrper(pub *mut gpujpeg_image_parameters);
unsafe impl Send for GpujpegImageParametersWarrper {}
unsafe impl Sync for GpujpegImageParametersWarrper {}

pub struct GpujpegEncoderInputWarapper(pub *mut gpujpeg_encoder_input);
unsafe impl Send for GpujpegEncoderInputWarapper {}
unsafe impl Sync for GpujpegEncoderInputWarapper {}

pub struct GpujpegEncoderWarpper(pub *mut gpujpeg_encoder);
unsafe impl Send for GpujpegEncoderWarpper {}
unsafe impl Sync for GpujpegEncoderWarpper {}
