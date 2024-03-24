use crate::sandsim::particle::*;
use crate::sandsim::grid::*;

use std::collections::HashMap;

pub struct BrushSettings {
    pub size: i32,
    pub brush_type: BrushType,
    pub callback: fn((i32, i32)) -> Cell,
    pub probability: f32,
}

pub enum BrushType {
    Circle,
    // Square,
}

impl BrushSettings {
    pub fn new(size: i32, probability: f32, brush_type: BrushType, callback: fn(Position) -> Cell) -> BrushSettings {
        BrushSettings {
            size,
            brush_type,
            probability,
            callback,
        }
    }
}

pub fn make_default_brush_settings_map() -> HashMap<ParticleId, BrushSettings> {
    let mut brush_settings_map = HashMap::new();
    brush_settings_map.insert(SAND_ID, BrushSettings::new(2, 0.5, BrushType::Circle, |_pos| Sand::boxed()));
    brush_settings_map.insert(WOOD_ID, BrushSettings::new(2, 0.99, BrushType::Circle, |_pos| Wood::boxed()));
    brush_settings_map.insert(EMPTY_ID, BrushSettings::new(2, 1., BrushType::Circle, |_pos| Empty::boxed()));

    brush_settings_map
}