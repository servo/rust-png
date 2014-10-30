// Copyright 2013 The Servo Project Developers. See the COPYRIGHT
// file at the top-level directory of this distribution.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

#![crate_name = "png"]
#![crate_type = "rlib"]

extern crate libc;

use libc::{c_int, size_t};
use std::mem;
use std::io;
use std::io::File;
use std::ptr;
use std::slice;

pub mod ffi;


pub enum PixelsByColorType {
    K8(Vec<u8>),
    KA8(Vec<u8>),
    RGB8(Vec<u8>),
    RGBA8(Vec<u8>),
}

pub struct Image {
    pub width: u32,
    pub height: u32,
    pub pixels: PixelsByColorType,
}

// This intermediate data structure is used to read
// an image data from 'offset' position, and store it
// to the data vector.
struct ImageData<'a> {
    data: &'a [u8],
    offset: uint,
}

pub fn is_png(image: &[u8]) -> bool {
    unsafe {
        ffi::png_sig_cmp(image.as_ptr(), 0, 8) == 0
    }
}

pub extern fn read_data(png_ptr: *mut ffi::png_struct, data: *mut u8, length: size_t) {
    unsafe {
        let io_ptr = ffi::png_get_io_ptr(png_ptr);
        let image_data: &mut ImageData = mem::transmute(io_ptr);
        let len = length as uint;
        slice::raw::mut_buf_as_slice(data, len, |buf| {
            let end_pos = std::cmp::min(image_data.data.len()-image_data.offset, len);
            let src = image_data.data.slice(image_data.offset, image_data.offset+end_pos);
            ptr::copy_memory(buf.as_mut_ptr(), src.as_ptr(), src.len());
            image_data.offset += end_pos;
        });
    }
}

pub fn load_png(path: &Path) -> Result<Image,String> {
    let mut reader = match File::open_mode(path, io::Open, io::Read) {
        Ok(r) => r,
        Err(e) => return Err(format!("could not open file: {}", e.desc)),
    };
    let buf = match reader.read_to_end() {
        Ok(b) => b,
        Err(e) => return Err(format!("could not read file: {}", e.desc))
    };
    load_png_from_memory(buf.as_slice())
}

pub fn load_png_from_memory(image: &[u8]) -> Result<Image,String> {
    unsafe {
        let mut png_ptr = ffi::png_create_read_struct(&*ffi::png_get_header_ver(ptr::null_mut()),
                                                      ptr::null_mut(),
                                                      ptr::null_mut(),
                                                      ptr::null_mut());
        if png_ptr.is_null() {
            return Err("could not create read struct".to_string());
        }
        let mut info_ptr = ffi::png_create_info_struct(png_ptr);
        if info_ptr.is_null() {
            ffi::png_destroy_read_struct(&mut png_ptr, ptr::null_mut(), ptr::null_mut());
            return Err("could not create info struct".to_string());
        }
        let res = ffi::setjmp(ffi::pngshim_jmpbuf(png_ptr));
        if res != 0 {
            ffi::png_destroy_read_struct(&mut png_ptr, &mut info_ptr, ptr::null_mut());
            return Err("error reading png".to_string());
        }

        let mut image_data = ImageData {
            data: image,
            offset: 0,
        };

        ffi::png_set_read_fn(png_ptr, mem::transmute(&mut image_data), read_data);
        ffi::png_read_info(png_ptr, info_ptr);

        let width = ffi::png_get_image_width(png_ptr, info_ptr);
        let height = ffi::png_get_image_height(png_ptr, info_ptr);
        let color_type = ffi::png_get_color_type(png_ptr, info_ptr);

        // convert palette and grayscale to rgb
        match color_type as c_int {
            ffi::COLOR_TYPE_PALETTE => {
                ffi::png_set_palette_to_rgb(png_ptr);
            }
            ffi::COLOR_TYPE_GRAY | ffi::COLOR_TYPE_GRAY_ALPHA => {
                ffi::png_set_gray_to_rgb(png_ptr);
            }
            _ => {}
        }

        // add alpha channels
        ffi::png_set_add_alpha(png_ptr, 0xff, ffi::FILLER_AFTER);
        if ffi::png_get_valid(png_ptr, info_ptr, ffi::INFO_tRNS as u32) != 0 {
            ffi::png_set_tRNS_to_alpha(png_ptr);
        }

        ffi::png_set_packing(png_ptr);
        ffi::png_set_interlace_handling(png_ptr);
        ffi::png_read_update_info(png_ptr, info_ptr);

        let updated_bit_depth = ffi::png_get_bit_depth(png_ptr, info_ptr);
        let updated_color_type = ffi::png_get_color_type(png_ptr, info_ptr);

        let (color_type, pixel_width) = match (updated_color_type as c_int, updated_bit_depth) {
            (ffi::COLOR_TYPE_RGB, 8) |
            (ffi::COLOR_TYPE_RGBA, 8) |
            (ffi::COLOR_TYPE_PALETTE, 8) => (RGBA8, 4),
            (ffi::COLOR_TYPE_GRAY, 8) => (K8, 1),
            (ffi::COLOR_TYPE_GA, 8) => (KA8, 2),
            _ => panic!("color type not supported"),
        };

        let mut image_data = Vec::from_elem((width * height * pixel_width) as uint, 0u8);
        let image_buf = image_data.as_mut_ptr();
        let mut row_pointers: Vec<*mut u8> = Vec::from_fn(height as uint, |idx| {
            image_buf.offset((((width * pixel_width) as uint) * idx) as int)
        });

        ffi::png_read_image(png_ptr, row_pointers.as_mut_ptr());

        ffi::png_destroy_read_struct(&mut png_ptr, &mut info_ptr, ptr::null_mut());

        Ok(Image {
            width: width,
            height: height,
            pixels: color_type(image_data),
        })
    }
}

pub extern fn write_data(png_ptr: *mut ffi::png_struct, data: *mut u8, length: size_t) {
    unsafe {
        let io_ptr = ffi::png_get_io_ptr(png_ptr);
        let writer: &mut &mut io::Writer = mem::transmute(io_ptr);
        slice::raw::buf_as_slice(&*data, length as uint, |buf| {
            match writer.write(buf) {
                Err(e) => panic!("{}", e.desc),
                _ => {}
            }
        });
    }
}

pub extern fn flush_data(png_ptr: *mut ffi::png_struct) {
    unsafe {
        let io_ptr = ffi::png_get_io_ptr(png_ptr);
        let writer: &mut &mut io::Writer = mem::transmute(io_ptr);
        match writer.flush() {
            Err(e) => panic!("{}", e.desc),
            _ => {}
        }
    }
}

pub fn store_png(img: &mut Image, path: &Path) -> Result<(),String> {
    let mut file = match File::create(path) {
        Ok(f) => f,
        Err(e) => return Err(format!("{}", e))
    };

    let mut writer = &mut file as &mut io::Writer;

    // Box it again because a &Trait is too big to fit in a void*.
    let writer = &mut writer;

    unsafe {
        let mut png_ptr = ffi::png_create_write_struct(&*ffi::png_get_header_ver(ptr::null_mut()),
                                                       ptr::null_mut(),
                                                       ptr::null_mut(),
                                                       ptr::null_mut());
        if png_ptr.is_null() {
            return Err("could not create write struct".to_string());
        }
        let mut info_ptr = ffi::png_create_info_struct(png_ptr);
        if info_ptr.is_null() {
            ffi::png_destroy_write_struct(&mut png_ptr, ptr::null_mut());
            return Err("could not create info struct".to_string());
        }
        let res = ffi::setjmp(ffi::pngshim_jmpbuf(png_ptr));
        if res != 0 {
            ffi::png_destroy_write_struct(&mut png_ptr, &mut info_ptr);
            return Err("error writing png".to_string());
        }

        ffi::png_set_write_fn(png_ptr, mem::transmute(writer), write_data, flush_data);

        let (bit_depth, color_type, pixel_width, image_buf) = match img.pixels {
            RGB8(ref mut pixels) => (8, ffi::COLOR_TYPE_RGB, 3, pixels.as_mut_ptr()),
            RGBA8(ref mut pixels) => (8, ffi::COLOR_TYPE_RGBA, 4, pixels.as_mut_ptr()),
            K8(ref mut pixels) => (8, ffi::COLOR_TYPE_GRAY, 1, pixels.as_mut_ptr()),
            KA8(ref mut pixels) => (8, ffi::COLOR_TYPE_GA, 2, pixels.as_mut_ptr()),
        };

        ffi::png_set_IHDR(png_ptr, info_ptr, img.width, img.height, bit_depth, color_type,
                          ffi::INTERLACE_NONE, ffi::COMPRESSION_TYPE_DEFAULT, ffi::FILTER_NONE);

        let mut row_pointers: Vec<*mut u8> = Vec::from_fn(img.height as uint, |idx| {
            image_buf.offset((((img.width * pixel_width) as uint) * idx) as int)
        });
        ffi::png_set_rows(png_ptr, info_ptr, row_pointers.as_mut_ptr());

        ffi::png_write_png(png_ptr, info_ptr, ffi::TRANSFORM_IDENTITY, ptr::null_mut());

        ffi::png_destroy_write_struct(&mut png_ptr, &mut info_ptr);
    }
    Ok(())
}

#[cfg(test)]
mod test {
    extern crate test;
    use self::test::bench;
    use self::test::fmt_bench_samples;
    use std::io;
    use std::io::File;

    use super::{ffi, load_png, load_png_from_memory, store_png};
    use super::{RGB8, RGBA8, K8, KA8, Image};

    #[test]
    fn test_valid_png() {
        let file = "test/servo-screenshot.png";
        let mut reader = match File::open_mode(&Path::new(file), io::Open, io::Read) {
            Ok(r) => r,
            Err(e) => panic!(e.desc),
        };

        let mut buf = Vec::from_elem(1024, 0u8);
        let count = reader.read(buf.slice_mut(0, 1024)).unwrap();
        assert!(count >= 8);
        unsafe {
            let res = ffi::png_sig_cmp(buf.as_ptr(), 0, 8);
            assert!(res == 0);
        }
    }

    fn load_rgba8(file: &'static str, w: u32, h: u32) {
        match load_png(&Path::new(file)) {
            Err(m) => panic!(m),
            Ok(image) => {
                assert_eq!(image.width, w);
                assert_eq!(image.height, h);
                match image.pixels {
                    RGBA8(_) => {}
                    _ => panic!("Expected RGBA8")
                }
            }
        }
    }

    #[test]
    fn test_load() {
        load_rgba8("test/servo-screenshot.png", 831, 624);

        test_store();
        load_rgba8("test/store.png", 10, 10);
    }

    #[test]
    fn test_load_grayscale() {
        // grayscale images should be decoded to rgba
        load_rgba8("test/gray.png", 100, 100);
    }

    fn bench_file_from_memory(file: &'static str, w: u32, h: u32, c: &'static str) {
        let mut reader = match File::open_mode(&Path::new(file), io::Open, io::Read) {
            Ok(r) => r,
            Err(e) => panic!("could not open '{}': {}", file, e.desc)
        };
        let buf = match reader.read_to_end() {
            Ok(b) => b,
            Err(e) => panic!(e)
        };
        let bs = bench::benchmark(|b| b.iter(|| {
            match load_png_from_memory(buf.as_slice()) {
                Err(m) => panic!(m),
                Ok(image) => {
                    let color_type = match image.pixels {
                        K8(_) => "K8",
                        KA8(_) => "KA8",
                        RGB8(_) => "RGB8",
                        RGBA8(_) => "RGBA8",
                    };
                    assert_eq!(color_type, c);
                    assert_eq!(image.width, w);
                    assert_eq!(image.height, h);
                }
            }
        }));
        println!("libpng load '{}': {}", file, fmt_bench_samples(&bs));
    }

    #[test]
    fn test_load_perf() {
        bench_file_from_memory("test/servo-screenshot.png", 831, 624, "RGBA8");
        bench_file_from_memory("test/mozilla-dinosaur-head-logo.png", 1300, 929, "RGBA8");
        bench_file_from_memory("test/rust-huge-logo.png", 4000, 4000, "RGBA8");
    }

    #[test]
    fn test_store() {
        let mut img = Image {
            width: 10,
            height: 10,
            pixels: RGB8(Vec::from_elem(10 * 10 * 3, 100u8)),
        };
        let res = store_png(&mut img, &Path::new("test/store.png"));
        assert!(res.is_ok());
    }
}
