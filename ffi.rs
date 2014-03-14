// Copyright 2013 The Servo Project Developers. See the COPYRIGHT
// file at the top-level directory of this distribution.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

#[allow(non_camel_case_types)];

use std::libc::{c_int, size_t, c_void, c_char};

pub static TRANSFORM_IDENTITY: c_int = 0;

pub static FILTER_NONE: c_int = 0;

pub static INTERLACE_NONE: c_int = 0;

pub static COMPRESSION_TYPE_DEFAULT: c_int = 0;

pub static COLOR_TYPE_GRAY: c_int = 0;
pub static COLOR_TYPE_RGB: c_int = 2;
pub static COLOR_TYPE_PALETTE: c_int = 3;
pub static COLOR_TYPE_GRAY_ALPHA: c_int = 4;
pub static COLOR_TYPE_GA: c_int = 4;
pub static COLOR_TYPE_RGB_ALPHA: c_int = 6;
pub static COLOR_TYPE_RGBA: c_int = 6;

pub static FILLER_AFTER: c_int = 1;
pub static INFO_tRNS: c_int = 0x0010;

pub type png_struct = c_void;
pub type png_info = c_void;

#[link(name = "png")]
#[link(name = "z")]
#[link(name = "shim")]
extern {
    // libc routines needed
    pub fn setjmp(env: *c_void) -> c_int;

    // shim routines
    pub fn pngshim_jmpbuf(pnt_ptr: *mut png_struct) -> *c_void;

    // libpng routines
    pub fn png_get_header_ver(png_ptr: *png_struct) -> *c_char;
    pub fn png_sig_cmp(sig: *u8, start: size_t, num_to_check: size_t) -> c_int;

    pub fn png_create_info_struct(png_ptr: *png_struct) -> *mut png_info;
    pub fn png_get_io_ptr(png_ptr: *png_struct) -> *mut c_void;
    pub fn png_set_sig_bytes(png_ptr: *mut png_struct, num_bytes: c_int);

    pub fn png_create_read_struct(user_png_ver: *c_char, error_ptr: *c_void, error_fn: *u8, warn_fn: *u8) -> *mut png_struct;
    pub fn png_destroy_read_struct(png_ptr_ptr: **png_struct, info_ptr_ptr: **png_info, end_info_ptr_ptr: **png_info);
    pub fn png_set_read_fn(png_ptr: *mut png_struct, io_ptr: *mut c_void, read_data_fn: extern "C" fn(*png_struct, *mut u8, size_t));
    pub fn png_read_info(png_ptr: *mut png_struct, info_ptr: *mut png_info);
    pub fn png_read_update_info(png_ptr: *mut png_struct, info_ptr: *mut png_info);
    pub fn png_read_image(png_ptr: *mut png_struct, row_pointers: **mut u8);
    pub fn png_read_png(png_ptr: *mut png_struct, info_ptr: *mut png_info, transforms: c_int, params: *c_void);

    pub fn png_create_write_struct(user_png_ver: *c_char, error_ptr: *c_void, error_fn: *u8, warn_fn: *u8) -> *mut png_struct;
    pub fn png_destroy_write_struct(png_ptr_ptr: **png_struct, info_ptr_ptr: **png_info);
    pub fn png_set_write_fn(png_ptr: *mut png_struct, io_ptr: *mut c_void, write_data_fn: extern "C" fn(*png_struct, *u8, size_t), output_flush_ptr: extern "C" fn(*png_struct));
    pub fn png_write_png(pnt_ptr: *mut png_struct, info_ptr: *mut png_info, transforms: c_int, params: *c_void); // ??

    pub fn png_get_IHDR(png_ptr: *png_struct, info_ptr: *png_info, width: *mut u32, height: *mut u32, bit_depth: *mut c_int, color_type: *mut c_int, interlace_method: *mut c_int, compression_method: *mut c_int, filter_method: *mut c_int) -> u32;
    pub fn png_get_pHYs(png_ptr: *png_struct, info_ptr: *png_info, res_x: *mut u32, res_y: *mut u32, unit_type: *mut c_int) -> u32;
    pub fn png_get_image_width(png_ptr: *png_struct, info_ptr: *png_info) -> u32;
    pub fn png_get_image_height(png_ptr: *png_struct, info_ptr: *png_info) -> u32;
    pub fn png_get_bit_depth(png_ptr: *png_struct, info_ptr: *png_info) -> u8;
    pub fn png_get_color_type(png_ptr: *png_struct, info_ptr: *png_info) -> u8;
    pub fn png_get_rows(png_ptr: *png_struct, info_ptr: *png_info) -> **u8;

    pub fn png_set_IHDR(png_ptr: *png_struct, info_ptr: *mut png_info, width: u32, height: u32, bit_depth: c_int, color_type: c_int, interlace_method: c_int, compression_method: c_int, filter_method: c_int);
    pub fn png_set_pHYs(png_ptr: *png_struct, info_ptr: *mut png_info, res_x: u32, res_y: u32, unit_type: c_int);
    pub fn png_set_rows(png_ptr: *png_struct, info_ptr: *mut png_info, row_pointers: **u8);

    pub fn png_set_packing(png_ptr: *mut png_struct);
    pub fn png_set_palette_to_rgb(png_ptr: *mut png_struct);
    pub fn png_set_expand_gray_1_2_4_to_8(png_ptr: *mut png_struct);
    pub fn png_set_tRNS_to_alpha(png_ptr: *mut png_struct);
    pub fn png_set_filler(png_ptr: *mut png_struct, val: u32, flag: c_int);
    pub fn png_set_interlace_handling(png_ptr: *mut png_struct);
}
