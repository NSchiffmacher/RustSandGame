use crate::sandsim::particle::*;
use crate::sandsim::grid::*;

use std::collections::HashMap;

pub struct BrushSettings {
    pub size: i32,
    pub brush_type: BrushType,
    pub callback: fn((i32, i32)) -> Particle,
    pub probability: f32,
}

pub enum BrushType {
    Circle,
    // Square,
}

impl BrushSettings {
    pub fn new(size: i32, probability: f32, brush_type: BrushType, callback: fn(Position) -> Particle) -> BrushSettings {
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
    brush_settings_map.insert(SAND_ID, BrushSettings::new(2, 0.5, BrushType::Circle, |position| Particle::new_sand(position)));
    brush_settings_map.insert(WOOD_ID, BrushSettings::new(2, 0.99, BrushType::Circle, |position| Particle::new_wood(position)));
    brush_settings_map.insert(EMPTY_ID, BrushSettings::new(2, 1., BrushType::Circle, |position| Particle::new_empty(position)));

    brush_settings_map
}