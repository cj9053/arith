use crate::new_structs::{p_Avg_Coscoeff, Rgbfloat, CVCS};
use crate::quant_ops::{scale_sat, smax};
use std::ops::Deref;

use array2::Array2;
use csc411_arith::index_of_chroma;
use csc411_image::{Read, Rgb, RgbImage, Write};
