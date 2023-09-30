use macroquad::color::{Color};
use crate::color_map_listed::{INFERNO_LUT, MAGMA_LUT};

pub trait ValueToColor {
    fn value_to_color(&self, value: u8, alpha: Option<u8>) -> Color;
}

pub struct ListedColorMap {
    lut: [[u8; 3]; 256]
}

impl ListedColorMap {
    pub fn new(lut: [[u8; 3]; 256]) -> Self {
        Self { lut }
    }
}

impl ValueToColor for ListedColorMap {
    fn value_to_color(&self, value: u8, alpha: Option<u8>) -> Color {
        Color::from_rgba(
            self.lut[value as usize][0],
            self.lut[value as usize][1],
            self.lut[value as usize][2],
            alpha.unwrap_or(255),
        )
    }
}

pub struct GrayColorMap {}

impl GrayColorMap {
    pub fn new() -> Self {
        Self {}
    }
}

impl ValueToColor for GrayColorMap {
    fn value_to_color(&self, value: u8, alpha: Option<u8>) -> Color {
        Color::from_rgba(
            value,
            value,
            value,
            alpha.unwrap_or(255),
        )
    }
}
