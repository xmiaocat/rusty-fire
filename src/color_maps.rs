use macroquad::color::{Color};
use crate::color_map_listed::{INFERNO_LUT, MAGMA_LUT};

pub trait ValueToColor {
    fn value_to_color(&self, value: u8, alpha: Option<u8>) -> Color;
}

pub struct Magma {
    lut: [[u8; 3]; 256]
}

impl Magma {
    pub fn new() -> Self {
        Self { lut: MAGMA_LUT }
    }
}

impl ValueToColor for Magma {
    fn value_to_color(&self, value: u8, alpha: Option<u8>) -> Color {
        Color::from_rgba(
            self.lut[value as usize][0],
            self.lut[value as usize][1],
            self.lut[value as usize][2],
            alpha.unwrap_or(255),
        )
    }
}

pub struct Inferno {
    lut: [[u8; 3]; 256]
}

impl Inferno {
    pub fn new() -> Self {
        Self { lut: INFERNO_LUT }
    }
}

impl ValueToColor for Inferno {
    fn value_to_color(&self, value: u8, alpha: Option<u8>) -> Color {
        Color::from_rgba(
            self.lut[value as usize][0],
            self.lut[value as usize][1],
            self.lut[value as usize][2],
            alpha.unwrap_or(255),
        )
    }
}

pub struct Gray {}

impl Gray {
    pub fn new() -> Self {
        Self {}
    }
}

impl ValueToColor for Gray {
    fn value_to_color(&self, value: u8, alpha: Option<u8>) -> Color {
        Color::from_rgba(
            value,
            value,
            value,
            alpha.unwrap_or(255),
        )
    }
}
