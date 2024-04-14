use sdl2::pixels::Color;

use crate::sandsim::grid::Position;

#[derive(Debug, Clone)]
pub enum ParticleAction {
    SetPosition{position: Position},
    KillParticle{position: Position},
    SetColor{color: Color},
}