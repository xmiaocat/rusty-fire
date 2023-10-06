mod configs;
mod defaults;
mod color_maps;
mod color_map_listed;
mod fire_handler;
mod cooling_maps;

use std::vec;
use noise::{Fbm, MultiFractal, Perlin};
use macroquad::window::{Conf, next_frame, request_new_screen_size, screen_height, screen_width};
use macroquad::texture::{Image, Texture2D, draw_texture};
use macroquad::text::draw_text;
use macroquad::color::{colors};
use macroquad::rand::{srand, rand, gen_range};
use macroquad::time::{get_fps};
use crate::color_map_listed::{INFERNO_LUT, MAGMA_LUT, PLASMA_LUT, VIRIDIS_LUT};
use crate::color_maps::{ValueToColor, GrayColorMap, ListedColorMap};
use crate::configs::FireConfigs;
use crate::cooling_maps::{initialise_cooling_map, update_cooling_map};
use crate::defaults::{DEFAULT_WINDOW_HEIGHT, DEFAULT_WINDOW_WIDTH};
use crate::fire_handler::{seed_fire, smooth_and_cool};

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
    // let mode = "debug";

    request_new_screen_size(1280f32, 390f32);

    // Load configurations
    let mut fire_configs = FireConfigs::default();

    // TODO: Properly handle colormaps
    fire_configs.set_color_map_name(String::from("gray"));
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
    let noise_function: Fbm<Perlin> = Fbm::<Perlin>::new(rand()).set_octaves(1);

    // Initialise buffers
    let mut fire_mask: Vec<bool> = Vec::with_capacity(2 * w);
    let mut buf = vec![0u8; w * h];
    let mut buf_new = vec![0u8; w * h];

    // Prepare fire mask
    for _x in 0..w {
        let rand_num = gen_range(0, 99);
        if rand_num < fire_configs.fill_percentage {
            fire_mask.push(true);
        } else {
            fire_mask.push(false);
        }
    }
    for x in 0..w {
        fire_mask.push(fire_mask[x]);
    }

    let mut cooling_map = initialise_cooling_map(
        w,
        h,
        &noise_function,
        fire_configs.cooling_map_configs.length_scale,
        fire_configs.cooling_map_configs.strength,
    );
    let mut cooling_map_debug = initialise_cooling_map(
        w,
        h,
        &noise_function,
        fire_configs.cooling_map_configs.length_scale,
        1.0,
    );

    // Initialise image and texture
    let mut image = Image::gen_image_color(w as u16, h as u16, colors::BLACK);
    let texture = Texture2D::from_image(&image);

    // Initialise image and texture for debuggung
    let mut image_debug = Image::gen_image_color(w as u16, h as u16, colors::BLACK);
    let texture_debug = Texture2D::from_image(&image);

    // Start fire
    seed_fire(&mut buf, w, h, &fire_mask);

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

        seed_fire(&mut buf_new, w, h, &fire_mask);

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
            &noise_function,
            fire_configs.cooling_map_configs.length_scale,
            fire_configs.cooling_map_configs.strength,
            ystart,
        );

        update_cooling_map(
            &mut cooling_map_debug,
            w,
            h,
            &noise_function,
            fire_configs.cooling_map_configs.length_scale,
            1.0,
            ystart,
        );

        ystart += fire_configs.cooling_map_configs.length_scale;

        // update image buffer
        buf.copy_from_slice(&buf_new);

        image_debug.update(
            (
                cooling_map_debug.iter().map(
                    |&val| color_map.value_to_color(val, None)
                ).collect::<Vec<_>>()
            ).as_slice()
        );
        texture_debug.update(&image_debug);
        draw_texture(&texture_debug, 640.0, 0.0, colors::WHITE);


        next_frame().await
    }
}
