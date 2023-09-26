use std::collections::VecDeque;
use noise::{Fbm, NoiseFn, Perlin};
use macroquad::window::{
    Conf, next_frame, screen_height, screen_width,
};
use macroquad::texture::{Image, Texture2D, draw_texture};
use macroquad::text::draw_text;
use macroquad::color::{Color, colors};
use macroquad::rand::{srand, rand};
use macroquad::time::{get_fps};

const DEFAULT_N_FIRE_ROW: usize = 50;
const DEFAULT_COOLING_LENGTH_SCALE: f64 = 0.02;
const DEFAULT_COOLING_STRENGTH: f64 = 0.15;

fn val_to_color(val: u8) -> Color {
    Color::from_rgba(
        val,
        val,
        val,
        255,
    )
}

fn add_heat_to_buffer(
    buf: &mut [u8],
    w: usize,
    h: usize,
    n_row: usize,
) {
    // add new fire points
    for x in 0..w {
        for j in 0..n_row {
            buf[x + (h - j - 1) * w] = 255;
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
            let cooling_val = *cooling_map.get(x + y * w).unwrap();
            if new_val > cooling_val {
                new_val -= cooling_val;
            } else {
                new_val = 0;
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

struct CoolingMapSettings {
    seed: Option<u32>,
    length_scale: f64,
    strength: f64,
}

impl CoolingMapSettings {
    fn new(seed: Option<u32>, length_scale: f64, strength: f64) -> Self {
        Self {
            seed,
            length_scale,
            strength,
        }
    }

    fn default() -> Self {
        Self {
            seed: None,
            length_scale: DEFAULT_COOLING_LENGTH_SCALE,
            strength: DEFAULT_COOLING_STRENGTH,
        }
    }
}



#[macroquad::main(conf)]
async fn main() {
    let w = screen_width() as usize;
    let h = screen_height() as usize;

    let n_fire_row = DEFAULT_N_FIRE_ROW;
    let cooling_map_settings = CoolingMapSettings::default();

    srand(42);
    let cm_seed = if cooling_map_settings.seed.is_some() {
        cooling_map_settings.seed.unwrap()
    } else {
        rand()
    };

    let rng: Fbm<Perlin> = Fbm::<Perlin>::new(cm_seed);

    let mut ystart: f64 = 0.0;

    let mut buf = vec![0u8; w * h];
    let mut buf_new = vec![0u8; w * h];
    let mut cm_buf = initialise_cooling_map_buffer(
        w,
        h,
        &rng,
        cooling_map_settings.length_scale,
        cooling_map_settings.strength,
    );

    let mut image = Image::gen_image_color(w as u16, h as u16, colors::BLACK);
    let texture = Texture2D::from_image(&image);

    add_heat_to_buffer(&mut buf, w, h, n_fire_row);

    loop {
        // std::thread::sleep(std::time::Duration::from_millis(200));

        // smooth buffer values
        smooth_and_cool(&buf, &mut buf_new, w, h, 1, &cm_buf);

        add_heat_to_buffer(&mut buf_new, w, h, n_fire_row);

        // convert buf2 to image by mapping values to colors
        image.update(
            (buf_new.iter().map(|&val| val_to_color(val)).collect::<Vec<_>>()).as_slice()
        );

        // update and draw texture
        texture.update(&image);
        draw_texture(&texture, 0.0, 0.0, colors::WHITE);

        draw_text(format!("FPS: {}", get_fps()).as_str(), 0., 16., 32., colors::WHITE);

        // update cooling map buffer
        update_cooling_map_buffer(
            &mut cm_buf,
            w,
            h,
            &rng,
            cooling_map_settings.length_scale,
            cooling_map_settings.strength,
            ystart,
        );
        ystart += cooling_map_settings.length_scale;

        // update image buffer
        buf.copy_from_slice(&buf_new);
        
        next_frame().await
    }
}
