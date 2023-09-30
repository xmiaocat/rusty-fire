mod configs;
mod defaults;
mod color_maps;
mod color_map_listed;

use std::collections::VecDeque;
use std::vec;
use noise::{Fbm, NoiseFn, Perlin};
use macroquad::window::{
    Conf, next_frame, screen_height, screen_width,
};
use macroquad::texture::{Image, Texture2D, draw_texture};
use macroquad::text::draw_text;
use macroquad::color::{Color, colors};
use macroquad::rand::{srand, rand, gen_range};
use macroquad::time::{get_fps};
use crate::color_map_listed::{INFERNO_LUT, MAGMA_LUT, PLASMA_LUT, VIRIDIS_LUT};
use crate::color_maps::{ValueToColor, GrayColorMap, ListedColorMap};
use crate::configs::FireConfigs;
use crate::defaults::{DEFAULT_WINDOW_HEIGHT, DEFAULT_WINDOW_WIDTH};

fn heat_buffer(
    buf: &mut [u8],
    w: usize,
    h: usize,
    fire_positions: &[bool],
) {
    // add new fire points
    for x in 0..w {
        if fire_positions[x] {
            buf[x + (h - 2) * w] = 255;
            buf[x + (h - 1) * w] = 255;
        }
    }
}

fn smooth_and_cool(
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

fn initialise_cooling_map(
    w: usize,
    h: usize,
    rng: &impl NoiseFn<f64, 2>,
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
            let n = rng.get([xoff, yoff]);
            let val = ((n * 0.5 + 0.5).clamp(0.0, 1.0).powf(3.0) * scale * 255.0).round() as u8;
            cm_buf.push_back(val);
        }
    }
    cm_buf
}

fn update_cooling_map(
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
        window_width: DEFAULT_WINDOW_WIDTH,
        window_height: DEFAULT_WINDOW_HEIGHT,
        fullscreen: false,
        window_resizable: false,
        ..Default::default()
    }
}

#[macroquad::main(conf)]
async fn main() {
    // Load configurations
    let mut fire_configs = FireConfigs::default();

    // TODO: Properly handle colormaps
    fire_configs.set_color_map_name(String::from("magma"));
    let color_map: Box<dyn ValueToColor> = match fire_configs.color_map_name.to_ascii_lowercase().as_str() {
        "gray" => Box::new(GrayColorMap::new()),
        "magma" => Box::new(ListedColorMap::new(MAGMA_LUT)),
        "inferno" => Box::new(ListedColorMap::new(INFERNO_LUT)),
        "plasma" => Box::new(ListedColorMap::new(PLASMA_LUT)),
        "viridis" => Box::new(ListedColorMap::new(VIRIDIS_LUT)),
        _ => Box::new(GrayColorMap::new()),
    };

    // Define convenience variables
    let w = screen_width() as usize;
    let h = screen_height() as usize;

    // Seed rngs
    if fire_configs.seed.is_some() {
        srand(fire_configs.seed.unwrap());
    }
    let cooling_map_rng: Fbm<Perlin> = Fbm::<Perlin>::new(rand());

    // Initialise buffers
    let mut fire_position: Vec<bool> = Vec::with_capacity(w);
    let mut buf = vec![0u8; w * h];
    let mut buf_new = vec![0u8; w * h];
    let mut cooling_map = initialise_cooling_map(
        w,
        h,
        &cooling_map_rng,
        fire_configs.cooling_map_configs.length_scale,
        fire_configs.cooling_map_configs.strength,
    );

    // Get fire positions
    for _ in 0..w {
        let rand_num = gen_range(0, 99);
        if rand_num < fire_configs.fill_percentage {
            fire_position.push(true);
        } else {
            fire_position.push(false);
        }
    }

    // Initialise image and texture
    let mut image = Image::gen_image_color(w as u16, h as u16, colors::BLACK);
    let texture = Texture2D::from_image(&image);

    // Start fire
    heat_buffer(&mut buf, w, h, &fire_position);

    // Initialise running variable
    let mut ystart: f64 = 0.0;
    loop {
        // std::thread::sleep(std::time::Duration::from_millis(200));

        // Perform smoothing and cooling
        smooth_and_cool(
            &buf,
            &mut buf_new,
            w,
            h,
            1,
            &cooling_map,
            fire_configs.base_height,
        );

        heat_buffer(&mut buf_new, w, h, &fire_position);

        // convert buf_new to image by mapping values to colors
        image.update(
            (
                buf_new.iter().map(
                    |&val| color_map.value_to_color(val, None)
                ).collect::<Vec<_>>()
            ).as_slice()
        );

        // update and draw texture
        texture.update(&image);
        draw_texture(&texture, 0.0, 0.0, colors::WHITE);

        // draw fps for debugging
        draw_text(format!("FPS: {}", get_fps()).as_str(), 0., 16., 32., colors::WHITE);

        // update cooling map buffer
        update_cooling_map(
            &mut cooling_map,
            w,
            h,
            &cooling_map_rng,
            fire_configs.cooling_map_configs.length_scale,
            fire_configs.cooling_map_configs.strength,
            ystart,
        );
        ystart += fire_configs.cooling_map_configs.length_scale;

        // update image buffer
        buf.copy_from_slice(&buf_new);
        
        next_frame().await
    }
}
