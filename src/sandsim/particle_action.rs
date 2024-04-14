use sdl2::pixels::Color;

use crate::sandsim::grid::Position;

pub enum ParticleAction {
    SetColor{color: Color},
    SetPosition{position: Position},
}