// use sdl2::pixels::Color;

use crate::sandsim::grid::Position;

#[derive(Debug, Clone)]
pub enum ParticleAction {
    //SetColor{color: Color},
    SetPosition{position: Position},
    KillParticle{position: Position},
}