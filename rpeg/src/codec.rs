#![allow(warnings)]
use crate::new_structs::{p_Avg_Coscoeff, Rgbfloat, CVCS};
use crate::quant_ops::{scale_sat, smax};
use std::ops::Deref;

use array2::Array2;
use csc411_arith::index_of_chroma;
use csc411_image::{Read, Rgb, RgbImage, Write};

//input for compression, convert to array2 and trim
//compartmentalize conversion and trim funcs  since decompression doesn't require trimming
//rewrite process_input func
//pass read image from main to codec.rs

pub fn process_input(input_img: Option<&str>) -> (Array2<csc411_image::Rgb>, u16) {
    let raw_img = RgbImage::read(input_img).unwrap();
    let mut img_w: u32 = raw_img.width;
    let mut img_h: u32 = raw_img.height;
    let denom = raw_img.denominator;

    //if width is not even, -1
    if &raw_img.width % 2 != 0 {
        img_w -= 1;
    }
    // println!("{}, Image width ", img_w);
    if &raw_img.height % 2 != 0 {
        img_h -= 1;
    }
    // println!("{}, Image width ", img_h);

    //convert image to array2 & trims odd width || height or both
    let vals = raw_img.pixels.clone();
    let converted_img =
        Array2::<Rgb>::from_row_major(raw_img.width as usize, raw_img.height as usize, vals)
            .unwrap();
    let trimmed_img_val: Vec<Rgb> = converted_img
        .iter_row_major()
        .filter_map(|(col, row, pixel)| {
            if col < img_w as usize && row < img_h as usize {
                Some(pixel.clone())
            } else {
                None
            }
        })
        .collect();
    (
        Array2::<csc411_image::Rgb>::from_row_major(
            img_w as usize,
            img_h as usize,
            trimmed_img_val,
        )
        .expect("REASON"),
        denom,
    )
}
//goal is to extract the rgb values retried from previous function and noramlize it by dividing by denom
//iterate through the values, get the mutable reference and perform edits on red green and blue
//start by
pub fn rgb2float(trimmed_img: &Array2<csc411_image::Rgb>, denom: u16) -> Array2<Rgbfloat> {
    //init new arr

    let mut converted = Array2::new(
        trimmed_img.width,
        trimmed_img.height,
        Rgbfloat {
            red: 0.0,   // Default value for red
            green: 0.0, // Default value for green
            blue: 0.0,  // Default value for blue
        },
    );

    for (col, row, pixel) in trimmed_img.iter_row_major() {
        if let Some(val) = converted.get_mut(col, row) {
            let red = pixel.red as f64 / denom as f64;
            let green = pixel.green as f64 / denom as f64;
            let blue = pixel.blue as f64 / denom as f64;
            *val = Rgbfloat { red, green, blue };
        }
    }
    converted
}

//take in array2<rgbfloat> and convert to rgb
pub fn float2rgb(rgbfloat_arr: &Array2<Rgbfloat>, denom: u16) -> Array2<Rgb> {
    let mut converted = Array2::new(
        rgbfloat_arr.width,
        rgbfloat_arr.height,
        Rgb {
            red: 0,
            green: 0,
            blue: 0,
        },
    );

    for (col, row, pixel) in rgbfloat_arr.iter_row_major() {
        if let Some(val) = converted.get_mut(col, row) {
            let red = (pixel.red * denom as f64) as u16;
            println!("{} RED", red);
            let green = (pixel.green * denom as f64) as u16;
            println!("{} gre", green);

            let blue = (pixel.blue * denom as f64) as u16;
            println!("{} bl", blue);

            *val = Rgb { red, green, blue };
        }
    }
    converted
}

//convert Rgb float to component video color space should return
//y (range = real num from 0-1), contains pb and pr: y= brightness of a color while pb&pr are color differences
//pb = B-Y while pr = R-Y, range is between +-0.5
//x&z vals represent chromacity

//returns y,pb, and pr for each pixel
pub fn rgbf2component(rgb_float: Array2<Rgbfloat>) -> Array2<CVCS> {
    let mut component_vals = Array2::new(
        rgb_float.width,
        rgb_float.height,
        CVCS {
            y: 0.0,
            pb: 0.0,
            pr: 0.0,
        },
    );
    for (col, row, pixel) in rgb_float.iter_row_major() {
        if let Some(val) = component_vals.get_mut(col, row) {
            let y = (0.299 * pixel.red) + (0.587 * pixel.green) + (0.114 * pixel.blue);
            let pb = (-0.168736 * pixel.red) - (0.331264 * pixel.green) + (0.5 * pixel.blue);
            let pr = (0.5 * pixel.red) - (0.418688 * pixel.green) - (0.081312 * pixel.blue);
            *val = CVCS { y, pb, pr };
            // println!("{} y, {} pb, {},", y, pb, pr);
        }
    }
    component_vals
}

pub fn component2rgbf(component_val: Array2<CVCS>) -> Array2<Rgbfloat> {
    let mut converted = Array2::new(
        component_val.width,
        component_val.height,
        Rgbfloat {
            red: 0.0,
            green: 0.0,
            blue: 0.0,
        },
    );
    for (col, row, pixel) in component_val.iter_row_major() {
        if let Some(val) = converted.get_mut(col, row) {
            let red = (1.0 * pixel.y) + (0.0 * pixel.pb) + (1.402 * pixel.pr);
            let green = (1.0 * pixel.y) - (0.344136 * pixel.pb) - (0.714136 * pixel.pr);
            let blue = (1.0 * pixel.y) + (1.772 * pixel.pb) + (0.0 * pixel.pr);
            *val = Rgbfloat { red, green, blue };
        }
    }
    converted
}
//discrete cosine transform
pub fn dct(p1: &CVCS, p2: &CVCS, p3: &CVCS, p4: &CVCS) -> (f64, f64, f64, f64) {
    let a = (p4.y + p3.y + p2.y + p1.y) / 4.0;
    let b = (p4.y + p3.y - p2.y - p1.y) / 4.0;
    let c = (p4.y - p3.y + p2.y - p1.y) / 4.0;
    let d = (p4.y - p3.y - p2.y + p1.y) / 4.0;

    (a, b, c, d)
}

// fn inverse_dct(a,b,c,d) -> (f64,f64,f64,f64){
//     let y1 = a - b - c + d;
//     let y2 = a - b + c - d;
//     let y3 = a + b - c - d;
//     let y4 = a + b + c + d;

//     todo!();

//     (y1, y2, y3, y4)
// }

//function that averages pb_pr
fn pb_pr_avg(p1: &CVCS, p2: &CVCS, p3: &CVCS, p4: &CVCS) -> (f64, f64) {
    let block_avg_pb = (p1.pb + p2.pb + p3.pb + p4.pb) / 4 as f64;
    let block_avg_pr = (p1.pr + p2.pr + p3.pr + p4.pr) / 4 as f64;

    (block_avg_pb, block_avg_pr)
}

//acquire 2x2 block and and compute pb&pr average as well as cosine coeff
pub fn block_ops(component_val: Array2<CVCS>) -> Array2<p_Avg_Coscoeff> {
    let mut block_val = Array2::new(
        component_val.width / 2,
        component_val.height / 2,
        p_Avg_Coscoeff {
            pb_bar: 0.0,
            pr_bar: 0.0,
            a: 0.0,
            b: 0.0,
            c: 0.0,
            d: 0.0,
        },
    );

    for row in (0..component_val.height).step_by(2) {
        for col in (0..component_val.width).step_by(2) {
            if let (Some(p1), Some(p2), Some(p3), Some(p4)) = (
                component_val.get(row, col),
                component_val.get(row, col + 1),
                component_val.get(row + 1, col),
                component_val.get(row + 1, col + 1),
            ) {
                ///pb_pr avg
                let (pb_bar, pr_bar) = pb_pr_avg(p1, p2, p3, p4);
                ///generate cosine coeffs from y values of 4 pixels
                let (a, b, c, d) = dct(p1, p2, p3, p4);
                if let Some(val) = block_val.get_mut(col, row) {
                    *val = p_Avg_Coscoeff {
                        pb_bar,
                        pr_bar,
                        a,
                        b,
                        c,
                        d,
                    }
                }
            }
        }
    }
    block_val
}

//use arith::chroma of index(index of chroma) for quantization of pb&pr
//implement function to quantize a,b,c,d

pub fn quantize_a_b_c_d(a: f64, b: f64, c: f64, d: f64) -> (u16, i32, i32, i32) {
    static COSINE_FORCE: f32 = 0.3;
    let quant_a = (a * 511 as f64).round() as u16;
    let quant_b = (scale_sat(b as f32, COSINE_FORCE) * smax(5) as f32).floor() as i32;
    let quant_c = (scale_sat(c as f32, COSINE_FORCE) * smax(5) as f32).floor() as i32;
    let quant_d = (scale_sat(d as f32, COSINE_FORCE) * smax(5) as f32).floor() as i32;

    (quant_a, quant_b, quant_c, quant_d)
}

pub fn quantize_pb_pr(pb_bar: f64, pr_bar: f64) -> (u16, u16) {
    let quant_pr = index_of_chroma(pr_bar as f32);
    let quant_pb = index_of_chroma(pr_bar as f32);

    (quant_pr as u16, quant_pb as u16)
}
//convert the floats into ints by utilizing bits (pretty much assigning an int value per decimal point)
//refer to page 11

//bit packer func

//pass file into compressor func then process img
pub fn compress(filename: Option<&str>) {
    let processed_img = process_input(filename);
}
pub fn decompress(filename: Option<&str>) {
    todo!();
}
