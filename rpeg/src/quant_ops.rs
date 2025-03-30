//copied from class notes:

pub fn scale_sat(x: f32, max_magnitude: f32) -> f32 {
    if x > max_magnitude {
        return max_magnitude;
    } else if x < -max_magnitude {
        return -max_magnitude;
    } else {
        return x / max_magnitude;
    }
}

pub fn smax(bits: i32) -> i32 {
    return (1 << bits) / 2 - 1;
}

// floor(scale_sat(b, COSINE_FORCE) * smax(5) as f32);
