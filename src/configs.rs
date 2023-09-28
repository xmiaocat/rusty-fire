use crate::defaults::*;

pub struct FireConfigs {
    pub seed: Option<u64>,
    pub fill_percentage: u8,
    pub base_height: usize,
    pub color_map_name: String,
    pub cooling_map_configs: CoolingMapConfigs,
}

impl FireConfigs {
    fn new(
        seed: Option<u64>,
        fill_percentage: u8,
        base_height: usize,
        color_map_name: String,
        cooling_map_configs: CoolingMapConfigs,
    ) -> Self {
        Self {
            seed,
            fill_percentage,
            base_height,
            color_map_name,
            cooling_map_configs,
        }
    }

    pub fn default() -> Self {
        Self {
            seed: None,
            fill_percentage: DEFAULT_FILL_PERCENTAGE,
            base_height: DEFAULT_FIRE_BASE_HEIGHT,
            color_map_name: String::from("Gray"),
            cooling_map_configs: CoolingMapConfigs::default(),
        }
    }

    pub fn set_color_map_name(&mut self, color_map_name: String) {
        self.color_map_name = color_map_name;
    }
}

pub struct CoolingMapConfigs {
    pub length_scale: f64,
    pub strength: f64,
}

impl CoolingMapConfigs {
    fn new(length_scale: f64, strength: f64) -> Self {
        Self {
            length_scale,
            strength,
        }
    }

    fn default() -> Self {
        Self {
            length_scale: DEFAULT_COOLING_LENGTH_SCALE,
            strength: DEFAULT_COOLING_STRENGTH
        }
    }
}