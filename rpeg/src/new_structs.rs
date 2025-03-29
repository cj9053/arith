#![allow(warnings)]
#[derive(Clone, Debug)]
//needed to handle normalized rgb val
pub struct Rgbfloat {
    pub red: f64,
    pub green: f64,
    pub blue: f64,
}
#[derive(Clone, Debug)]
//component video color space (CVCS)
pub struct CVCS {
    pub y: f64,
    pub pb: f64,
    pub pr: f64,
}

#[derive(Clone, Debug)]
//struct to handle pb_bar,pr_bar, a,b,c,d
pub struct p_Avg_Coscoeff {
    pub pb_bar: f64,
    pub pr_bar: f64,
    pub a: f64,
    pub b: f64,
    pub c: f64,
    pub d: f64,
}
