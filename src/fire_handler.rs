use std::collections::VecDeque;

pub fn seed_fire(
    buf: &mut [u8],
    w: usize,
    h: usize,
    fire_mask: &[bool],
) {
    let nrow = fire_mask.len() / w;
    // add new fire points
    for y in 0..nrow {
        for x in 0..w {
            if fire_mask[x + y * w] {
                buf[x + (h - y - 1) * w] = 255;
            }
        }
    }
}

pub fn smooth_and_cool(
    original: &[u8],
    new: &mut [u8],
    w: usize,
    h: usize,
    yshift: usize,
    cooling_map: &VecDeque<u8>,
    fire_height: usize,
) -> () {
    for x in 1..(w - 1) {
        for y in 1..(h - 1) {
            if y < yshift {
                continue;
            }
            let mut new_val = u8::try_from((
                u16::from(original[x + (y - 1) * w])
                    + u16::from(original[x + (y + 1) * w])
                    + u16::from(original[(x - 1) + y * w])
                    + u16::from(original[(x + 1) + y * w])
                // + u16::from(original[x + y * w])
            ) / 4).unwrap();
            if y < (h - fire_height) {
                let cooling_val = *cooling_map.get(x + y * w).unwrap();
                if new_val > cooling_val {
                    new_val -= cooling_val;
                } else {
                    new_val = 0;
                }
            }

            new[x + (y - yshift) * w] = new_val;
        }
    }
}