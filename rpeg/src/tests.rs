#![allow(warnings)]
use crate::codec;
use crate::new_structs::{Rgbfloat, CVCS};
use crate::quant_ops;
use array2::Array2;
use csc411_image::{Read, Rgb, RgbImage, Write};
use rand::Rng;
//generate multiple random inputs, run it through the original functions and its inverse, compare results
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_process_input() {
        //79x75 img
        let file: String = "odddims.ppm".to_string();
        let (arr, denom) = codec::process_input(Some(&file));
        let width = arr.width;
        println!("{} width", width);

        let height = arr.height;
        println!("{} height", height);
        let den = denom;

        assert_eq!(width, 78);
        assert_eq!(height, 74);
        assert_eq!(den, 255);
        //cargo test -- --nocapture to show println statements in testing
    }

    #[test]
    //create a 2x2 array to simulate an image, set each element with "Rgb vals" then normalize by dividing by denom
    //compare func output to expected val
    fn test_rgb2float() {
        let denom: u16 = 255;
        let mut sim_img = Array2::new(
            2,
            2,
            Rgb {
                red: 255,
                green: 255,
                blue: 255,
            },
        );

        sim_img.data[0] = Rgb {
            red: 100,
            green: 150,
            blue: 200,
        };
        sim_img.data[1] = Rgb {
            red: 130,
            green: 180,
            blue: 110,
        };
        sim_img.data[2] = Rgb {
            red: 120,
            green: 220,
            blue: 190,
        };
        sim_img.data[3] = Rgb {
            red: 230,
            green: 245,
            blue: 210,
        };

        //once result acquired, check if expected vals for each pix matches
        let result = codec::rgb2float(&sim_img, denom);
        let expected_values = vec![
            (
                (100.0 / denom as f64),
                (150.0 / denom as f64),
                (200.0 / denom as f64),
            ),
            (
                (130.0 / denom as f64),
                (180.0 / denom as f64),
                (110.0 / denom as f64),
            ),
            (
                (120.0 / denom as f64),
                (220.0 / denom as f64),
                (190.0 / denom as f64),
            ),
            (
                (230.0 / denom as f64),
                (245.0 / denom as f64),
                (210.0 / denom as f64),
            ),
        ];

        for (i, value) in result.data.iter().enumerate() {
            let value_tuple = (value.red, value.green, value.blue);
            println!(
                "\nReturned Val: {:?}, \n Expected Val: {:?}",
                value, expected_values[i]
            );
            assert_eq!(
                format!("{:?}", value_tuple),
                format!("{:?}", expected_values[i]),
                "Mismatch at index {}",
                i
            );
        }
        //run and test rgbfloat back to rgb
    }

    //run and test rgbfloat, should return denormalized rgb values
    /// inverse of rgb2float
    #[test]
    fn test_rgbfloat2rgb() {
        let denom: u16 = 255;
        let mut sim_img = Array2::new(
            2,
            2,
            Rgb {
                red: 255,
                green: 255,
                blue: 255,
            },
        );

        sim_img.data[0] = Rgb {
            red: 100,
            green: 150,
            blue: 200,
        };
        sim_img.data[1] = Rgb {
            red: 130,
            green: 180,
            blue: 110,
        };
        sim_img.data[2] = Rgb {
            red: 120,
            green: 220,
            blue: 190,
        };
        sim_img.data[3] = Rgb {
            red: 230,
            green: 245,
            blue: 210,
        };
        println!("float2rgb pix 1 red: {}", sim_img.data[0].red);
        let result = codec::rgb2float(&sim_img, denom);

        let rgbfloat2rgb_out = codec::float2rgb(&result, denom);

        for (col, row, pixel) in rgbfloat2rgb_out.iter_row_major() {
            let original_pixel = sim_img.get(col, row).expect("No value found");
            assert_eq!(
                (pixel.red, pixel.green, pixel.blue),
                (
                    original_pixel.red,
                    original_pixel.green,
                    original_pixel.blue
                ),
                "Mismatch at ({}, {})",
                col,
                row
            );
        }
    }

    #[test]
    fn test_rgb2component() {
        //test img
        //pix1 contents
        let red1: f64 = rand::rng().random_range(0.0..1.0);
        let green1: f64 = rand::rng().random_range(0.0..1.0);
        let blue1: f64 = rand::rng().random_range(0.0..1.0);
        //pix2contents
        let red2: f64 = rand::rng().random_range(0.0..1.0);
        let green2: f64 = rand::rng().random_range(0.0..1.0);
        let blue2: f64 = rand::rng().random_range(0.0..1.0);
        //pix3 contents
        let red3: f64 = rand::rng().random_range(0.0..1.0);
        let green3: f64 = rand::rng().random_range(0.0..1.0);
        let blue3: f64 = rand::rng().random_range(0.0..1.0);
        //pix4 contents
        let red4: f64 = rand::rng().random_range(0.0..1.0);
        let green4: f64 = rand::rng().random_range(0.0..1.0);
        let blue4: f64 = rand::rng().random_range(0.0..1.0);
        let mut sim_img = Array2::new(
            2,
            2,
            Rgbfloat {
                red: 0.0,
                green: 0.0,
                blue: 0.0,
            },
        );

        sim_img.data[0] = Rgbfloat {
            red: red1,
            green: green1,
            blue: blue1,
        };
        sim_img.data[1] = Rgbfloat {
            red: red2,
            green: green2,
            blue: blue2,
        };
        sim_img.data[2] = Rgbfloat {
            red: red3,
            green: green3,
            blue: blue3,
        };
        sim_img.data[3] = Rgbfloat {
            red: red4,
            green: green4,
            blue: blue4,
        };

        //create test for each pix
        //values in the vec are acquired from converted rgb values from rgb2float float test
        let result = codec::rgbf2component(sim_img);
        //init vals for pix1
        let y1 =
            //red
            (0.299 * red1)
            +
           //green
            (0.587 * green1)
            //blue
            + (0.114 * blue1);
        let pb1 =
            //red
            (-0.168736 * red1)
            -
           //green
            (0.331264 * green1)
            //blue
            + (0.5 * blue1);
        let pr1 =
            //red
            (0.5 * red1)
            -
           //green
            (0.418688 * green1)
            //blue
            - (0.081312 * blue1);

        //pixel2
        let y2 =
            //red
            (0.299 * red2)
            +
            //green
            (0.587 * green2)
            //blue
            + (0.114 * blue2);
        let pb2 =
            //red
            (-0.168736 * red2)
            -
            //green
            (0.331264 * green2)
            //blue
            + (0.5 * blue2);
        let pr2 =
            //red
            (0.5 * red2)
            -
            //green
            (0.418688 * green2)
            //blue
            - (0.081312 * blue2);

        //pixel3
        let y3 =
            //red
            (0.299 * red3)
            +
            //green
            (0.587 * green3)
            //blue
            + (0.114 * blue3);
        let pb3 =
            //red
            (-0.168736 * red3)
            -
            //green
            (0.331264 * green3)
            //blue
            + (0.5 * blue3);
        let pr3 =
            //red
            (0.5 * red3)
            -
            //green
            (0.418688 * green3)
            //blue
            - (0.081312 * blue3);

        //pixel 4
        let y4 =
            //red
            (0.299 * red4)
            +
            //green
            (0.587 * green4)
            //blue
            + (0.114 * blue4);
        let pb4 =
            //red
            (-0.168736 * red4)
            -
            //green
            (0.331264 * green4)
            //blue
            + (0.5 * blue4);
        let pr4 =
            //red
            (0.5 * red4)
            -
            //green
            (0.418688 * green4)
            //blue
            - (0.081312 * blue4);
        let expected_values = vec![
            (y1, pb1, pr1),
            (y2, pb2, pr2),
            (y3, pb3, pr3),
            (y4, pb4, pr4),
        ];

        // if let Some(result_iter) = result.data.iter().next() {
        //     assert_eq!(
        //         format!("{:?}", (result_iter.y, result_iter.pb, result_iter.pr)),
        //         format!("{:?}", expected_values[0])
        //     );
        // } else {
        //     panic!("No data");
        // }

        for (i, value) in result.data.iter().enumerate() {
            let value_tuple = (value.y, value.pb, value.pr);
            println!(
                "\nReturned Val: {:?}, \n Expected Val: {:?}",
                value, expected_values[i]
            );
            assert_eq!(
                format!("{:?}", value_tuple),
                format!("{:?}", expected_values[i]),
                "Mismatch at index {}",
                i
            );
        }
    }

    #[test]
    fn test_component2rgb() {
        let y1: f64 = rand::rng().random_range(0.0..1.0);
        let pb1: f64 = rand::rng().random_range(-0.5..0.5);
        let pr1: f64 = rand::rng().random_range(-0.5..0.5);

        let y2: f64 = rand::rng().random_range(0.0..1.0);
        let pb2: f64 = rand::rng().random_range(-0.5..0.5);
        let pr2: f64 = rand::rng().random_range(-0.5..0.5);

        let y3: f64 = rand::rng().random_range(0.0..1.0);
        let pb3: f64 = rand::rng().random_range(-0.5..0.5);
        let pr3: f64 = rand::rng().random_range(-0.5..0.5);

        let y4: f64 = rand::rng().random_range(0.0..1.0);
        let pb4: f64 = rand::rng().random_range(-0.5..0.5);
        let pr4: f64 = rand::rng().random_range(-0.5..0.5);

        let mut component_img = Array2::new(
            2,
            2,
            CVCS {
                y: 0.0,
                pb: 0.0,
                pr: 0.0,
            },
        );

        component_img.data[0] = CVCS {
            y: y1,
            pb: pb1,
            pr: pr1,
        };
        component_img.data[1] = CVCS {
            y: y2,
            pb: pb2,
            pr: pr2,
        };
        component_img.data[2] = CVCS {
            y: y3,
            pb: pb3,
            pr: pr3,
        };
        component_img.data[3] = CVCS {
            y: y4,
            pb: pb4,
            pr: pr4,
        };

        // Call the inverse function
        let result = codec::component2rgbf(component_img);

        // Expected RGB values
        let red1 = y1 + 1.402 * pr1;
        let green1 = y1 - 0.344136 * pb1 - 0.714136 * pr1;
        let blue1 = y1 + 1.772 * pb1;

        let red2 = y2 + 1.402 * pr2;
        let green2 = y2 - 0.344136 * pb2 - 0.714136 * pr2;
        let blue2 = y2 + 1.772 * pb2;

        let red3 = y3 + 1.402 * pr3;
        let green3 = y3 - 0.344136 * pb3 - 0.714136 * pr3;
        let blue3 = y3 + 1.772 * pb3;

        let red4 = y4 + 1.402 * pr4;
        let green4 = y4 - 0.344136 * pb4 - 0.714136 * pr4;
        let blue4 = y4 + 1.772 * pb4;

        let expected_values = vec![
            (red1, green1, blue1),
            (red2, green2, blue2),
            (red3, green3, blue3),
            (red4, green4, blue4),
        ];

        // Verify the results
        for (i, value) in result.data.iter().enumerate() {
            let value_tuple = (value.red, value.green, value.blue);
            println!(
                "\nReturned Val: {:?}, \n Expected Val: {:?}",
                value_tuple, expected_values[i]
            );
            assert!(
                (value_tuple.0 - expected_values[i].0).abs() < 1e-6
                    && (value_tuple.1 - expected_values[i].1).abs() < 1e-6
                    && (value_tuple.2 - expected_values[i].2).abs() < 1e-6,
                "Mismatch at index {}",
                i
            );
        }
    }

    #[test]
    fn test_dct() {
        let p1 = CVCS {
            y: 1.0,
            pb: 0.0,
            pr: 0.0,
        };
        let p2 = CVCS {
            y: 0.7,
            pb: 0.0,
            pr: 0.0,
        };
        let p3 = CVCS {
            y: 0.4,
            pb: 0.0,
            pr: 0.0,
        };
        let p4 = CVCS {
            y: 0.6,
            pb: 0.0,
            pr: 0.0,
        };

        let (a, b, c, d) = codec::dct(&p1, &p2, &p3, &p4);

        let result_vals = vec![a, b, c, d];

        // Expected results based on the DCT formula
        let e_a = (p1.y + p2.y + p3.y + p4.y) / 4.0;
        let e_b = (p4.y + p3.y - p2.y - p1.y) / 4.0;
        let e_c = (p4.y - p3.y + p2.y - p1.y) / 4.0;
        let e_d = (p4.y - p3.y - p2.y + p1.y) / 4.0;

        let expected_vals = vec![e_a, e_b, e_c, e_d];

        for (i, val) in result_vals.iter().enumerate() {
            assert_eq!(
                val, &expected_vals[i],
                "Mismatch at index {}: expected {}, got {}",
                i, expected_vals[i], val
            );
        }
    }

    #[test]
    fn test_dct_inverse() {}

    #[test]
    fn test_quant_ops() {
        // Input values
        let a = 0.5;
        let b = 0.5;
        let c = -0.8;
        let d = 0.1;

        // COSINE_FORCE constant
        const COSINE_FORCE: f32 = 0.3;

        // Manually calculate expected values
        let quant_a = (a * 511 as f64).round() as u16; // Scale and round "a" to fit in 9 bits (0-511)
        let smax_val = quant_ops::smax(5) as f32; // smax(5) = (1 << 5) / 2 - 1 = 15
        let quant_b = (quant_ops::scale_sat(b as f32, COSINE_FORCE) * smax_val).floor() as i32; // Expected: 2
        let quant_c = (quant_ops::scale_sat(c as f32, COSINE_FORCE) * smax_val).floor() as i32; // Expected: -4
        let quant_d = (quant_ops::scale_sat(d as f32, COSINE_FORCE) * smax_val).floor() as i32; // Expected: 0

        // Call the function
        let result = codec::quantize_a_b_c_d(a, b, c, d);

        // Assert expected values
        assert_eq!(result, (quant_a, quant_b, quant_c, quant_d));
    }
}
