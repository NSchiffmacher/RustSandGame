use sdl2::pixels::Color;

use crate::sandsim::grid::Position;
use crate::sandsim::particle::Particle;

#[derive(Clone)]
pub enum ParticleAction {
    SetPosition{position: Position},
    KillParticle{position: Position},
    SpawnParticle{callback: fn(Position) -> Particle, position: Position},
    SetColor{color: Color},
}