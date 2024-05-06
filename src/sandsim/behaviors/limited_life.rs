use rand::Rng;
use crate::sandsim::behaviors::*;
use crate::color::darken_color;

pub struct LimitedLife {
    elapsed_time: f64,

    spawn_probability: f64,
    spawn_callback: Option<fn(Position) -> Particle>,
    spawn_distance: Position,
    lifetime: f64,
}

impl Behavior for LimitedLife {
    fn update(&mut self, state: &mut ParticleState, dt: f64, grid: &mut Vec<Vec<ParticleId>>, _behaviors_grid: &mut Vec<Vec<BehaviorId>>) -> Vec<ParticleAction> {
        self.elapsed_time = self.lifetime.min(self.elapsed_time + dt);

        let t = (self.elapsed_time / self.lifetime) as f32; // t = 0 => start, t = 1 => end
        let lightness = 100. * (1. - t);
        let color = darken_color(state.color, lightness);

        let mut actions = vec![ParticleAction::SetColor { color }];

        if self.elapsed_time >= self.lifetime {
            actions.push(ParticleAction::KillParticle { position: state.position });
            if self.spawn_callback.is_some() && rand::thread_rng().gen_range(0.0..=1.0) <= self.spawn_probability {
                let width = grid[0].len();
                let height = grid.len();
                actions.push(ParticleAction::SpawnParticle {
                    callback: self.spawn_callback.unwrap(),
                    position: self.random_position(state.position, width as i32, height as i32),
                })
            }
        }

        actions
    }

    fn get_id(&self) -> BehaviorId {
        LIMITED_LIFE_ID
    }
}

impl LimitedLife {
    pub fn boxed(lifetime: f64) -> Box<dyn Behavior> {
        Box::new(Self {
            elapsed_time: 0.,
            lifetime,

            spawn_probability: 0.,
            spawn_callback: None,
            spawn_distance: (0, 0),
        })
    }

    pub fn boxed_with_spawn(lifetime: f64,
                            spawn_probability: f64,
                            spawn_callback: fn(Position) -> Particle,
                            spawn_distance: Position, )
        -> Box<dyn Behavior> {
        Box::new(Self {
            elapsed_time: 0.,

            lifetime,
            spawn_probability,
            spawn_callback: Some(spawn_callback),
            spawn_distance,
        })
    }

    fn random_position(&self, central_position: Position, width: i32, height: i32) -> Position {
        let mut rng = rand::thread_rng();

        let dx = rng.gen_range(-self.spawn_distance.0..=self.spawn_distance.0);
        let dy = rng.gen_range(-self.spawn_distance.1..=self.spawn_distance.1);

        let new_x = (central_position.0 + dx).clamp(0, width - 1);
        let new_y = (central_position.1 + dy).clamp(0, height - 1);

        (new_x, new_y)
    }
}