use std::collections::VecDeque;
use noise::NoiseFn;

pub fn initialise_cooling_map(
    w: usize,
    h: usize,
    noise_function: &impl NoiseFn<f64, 2>,
    increment: f64,
    scale: f64,
) -> VecDeque<u8> {
    let mut xoff: f64;
    let mut yoff = 0.0;
    let mut cm_buf = VecDeque::with_capacity(w * h);
    for _y in 0..h {
        xoff = 0.0;
        yoff += increment;
        for _x in 0..w {
            xoff += increment;
            let n = noise_function.get([xoff, yoff]);
            let val = ((n * 0.5 + 0.5).clamp(0.0, 1.0).powf(1.0) * scale * 255.0).round() as u8;
            cm_buf.push_back(val);
        }
    }
    cm_buf
}

pub fn update_cooling_map(
    buf: &mut VecDeque<u8>,
    w: usize,
    h: usize,
    noise_function: &impl NoiseFn<f64, 2>,
    increment: f64,
    scale: f64,
    ystart: f64,
) {
    let mut xoff = 0.0;
    let yoff = ystart + increment * h as f64;
    for _x in 0..w {
        // remove first row
        buf.pop_front();

        xoff += increment;
        let n = noise_function.get([xoff, yoff]);
        let val = ((n * 0.5 + 0.5).clamp(0.0, 1.0).powf(1.0) * scale * 255.0).round() as u8;
        buf.push_back(val);
    }
}