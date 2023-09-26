use std::collections::VecDeque;
use noise::{Fbm, NoiseFn, Perlin};
use macroquad::window::{
    Conf, next_frame, screen_height, screen_width,
};
use macroquad::texture::{Image, Texture2D, draw_texture};
use macroquad::text::draw_text;
use macroquad::color::{Color, colors};
use macroquad::rand::{srand, rand, gen_range};
use macroquad::time::{get_fps};

fn val_to_color(val: u8) -> Color {
    Color::from_rgba(
        val,
        val,
        val,
        255,
    )
}

fn smooth(
    original: &[u8],
    new: &mut [u8],
    w: usize,
    h: usize,
    yshift: usize,
    cooling_map: Option<&VecDeque<u8>>,
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
            if cooling_map.is_some() {
                let cooling_val = *cooling_map.unwrap().get(x + y * w).unwrap();
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

fn initialise_cooling_map_buffer(
    w: usize,
    h: usize,
    rng: &impl NoiseFn<f64, 2>,
    increment: f64,
    scale: f64,
) -> VecDeque<u8> {
    let mut xoff = 0.0;
    let mut yoff = 0.0;
    let mut cm_buf = VecDeque::with_capacity(w * h);
    for _y in 0..h {
        xoff = 0.0;
        yoff += increment;
        for _x in 0..w {
            xoff += increment;
            let n = rng.get([xoff, yoff]);
            let val = ((n * 0.5 + 0.5).clamp(0.0, 1.0).powf(1.0) * scale * 255.0).round() as u8;
            cm_buf.push_back(val);
        }
    }
    cm_buf
}

fn update_cooling_map_buffer(
    buf: &mut VecDeque<u8>,
    w: usize,
    h: usize,
    rng: &impl NoiseFn<f64, 2>,
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
        let n = rng.get([xoff, yoff]);
        let val = ((n * 0.5 + 0.5).clamp(0.0, 1.0).powf(1.0) * scale * 255.0).round() as u8;
        buf.push_back(val);
    }
}

fn conf() -> Conf {
    Conf {
        window_title: "Fire Simulator".to_string(),
        window_width: 600,
        window_height: 400,
        fullscreen: false,
        window_resizable: false,
        ..Default::default()
    }
}

#[macroquad::main(conf)]
async fn main() {
    let w = screen_width() as usize;
    let h = screen_height() as usize;

    const N_FIRE_ROW: usize = 2;
    const SEED: u64 = 0;
    const NOISE_INCREMENT: f64 = 0.02;
    const NOISE_SCALE: f64 = 0.1;

    srand(SEED);
    let rng: Fbm<Perlin> = Fbm::<Perlin>::new(1);

    let mut ystart: f64 = 0.0;

    let mut buf = vec![0u8; w * h];
    let mut buf_new = vec![0u8; w * h];
    let mut cm_buf = initialise_cooling_map_buffer(w, h, &rng, NOISE_INCREMENT, NOISE_SCALE);

    let mut image = Image::gen_image_color(w as u16, h as u16, colors::BLACK);
    let texture = Texture2D::from_image(&image);

    loop {
        // std::thread::sleep(std::time::Duration::from_millis(200));

        // add new fire points
        for x in 0..w {
            for j in 0..N_FIRE_ROW {
                buf[x + (h - j - 1) * w] = 255;
            }
        }

        // smooth buffer values
        smooth(&buf, &mut buf_new, w, h, 1, Some(&cm_buf));

        // convert buf2 to image by mapping values to colors
        image.update(
            (buf_new.iter().map(|&val| val_to_color(val)).collect::<Vec<_>>()).as_slice()
        );

        // update and draw texture
        texture.update(&image);
        draw_texture(&texture, 0.0, 0.0, colors::WHITE);

        draw_text(format!("FPS: {}", get_fps()).as_str(), 0., 16., 32., colors::WHITE);

        // update cooling map buffer
        update_cooling_map_buffer(&mut cm_buf, w, h, &rng, NOISE_INCREMENT, NOISE_SCALE, ystart);
        ystart += NOISE_INCREMENT;

        // update image buffer
        buf.copy_from_slice(&buf_new);
        
        next_frame().await
    }
}
