use crate::sandsim::behaviors::*;
use crate::sandsim::grid::Position;

/// `SidewaysMotionFallback` is a struct that implements the `Behavior` trait.
/// This behavior is responsible for moving a particle sideways when its downward motion is blocked.
pub struct SidewaysMotionFallback {
    last_position: Position,
}

impl Behavior for SidewaysMotionFallback {
    fn get_id(&self) -> BehaviorId {
        SIDEWAY_MOTION_FALLBACK
    }

    fn update(&mut self, state: &mut ParticleState, _dt: f64, grid: &mut Vec<Vec<ParticleId>>, behaviors_grid: &mut Vec<Vec<BehaviorId>>) -> Vec<ParticleAction> {
        let mut actions = vec![];

        if Self::are_all_downward_positions_blocked(state, grid, behaviors_grid) {
            // Check if any of the sideways positions are empty
            if let Some(new_position) = Self::get_empty_or_airlike_sideways_position(state, grid, behaviors_grid) {
                
                // Swap particle IDs
                grid[new_position.1 as usize][new_position.0 as usize] = grid[state.position.1 as usize][state.position.0 as usize];
                grid[state.position.1 as usize][state.position.0 as usize] = EMPTY_ID;

                // Swap behaviors IDs
                let tmp = behaviors_grid[new_position.1 as usize][new_position.0 as usize];
                behaviors_grid[new_position.1 as usize][new_position.0 as usize] = behaviors_grid[state.position.1 as usize][state.position.0 as usize];
                behaviors_grid[state.position.1 as usize][state.position.0 as usize] = tmp;
                
                actions.push(ParticleAction::SetPosition { position: new_position });
                state.position = new_position;
            }
        }

        self.last_position = state.position;
        actions
    }
}

impl SidewaysMotionFallback {
    pub fn boxed(position: &Position) -> Box<dyn Behavior> {
        Box::new(Self {last_position: position.clone()})
    }

    fn is_empty_or_airlike(position: Position, grid: &Vec<Vec<ParticleId>>, behaviors_grid: &Vec<Vec<BehaviorId>>) -> bool {
        let grid_height = grid.len() as i32;
        let grid_width = grid[0].len() as i32;

        if position.0 < 0 || position.0 >= grid_width || position.1 < 0 || position.1 >= grid_height {
            return false;
        }

        let particle = grid[position.1 as usize][position.0 as usize];
        particle == EMPTY_ID || has_behavior(position, behaviors_grid, AIR_LIKE_ID)
    }

    fn are_all_downward_positions_blocked(state: &mut ParticleState, grid: &Vec<Vec<ParticleId>>, behaviors_grid: &Vec<Vec<BehaviorId>>) -> bool {
        let height = grid.len() as i32;
        if state.position.1 + 1 >= height {
            return true;
        }
        
        !Self::is_empty_or_airlike((state.position.0, state.position.1 + 1), grid, behaviors_grid) &&
        !Self::is_empty_or_airlike((state.position.0 - 1, state.position.1 + 1), grid, behaviors_grid) &&
        !Self::is_empty_or_airlike((state.position.0 + 1, state.position.1 + 1), grid, behaviors_grid)
    }

    fn get_empty_or_airlike_sideways_position(state: &mut ParticleState, grid: &Vec<Vec<ParticleId>>, behaviors_grid: &Vec<Vec<BehaviorId>>) -> Option<Position> {
        let dx = if rand::random() { -1 } else { 1 };
        if Self::is_empty_or_airlike((state.position.0 - dx, state.position.1), grid, behaviors_grid) {
            return Some((state.position.0 - dx, state.position.1));
        } else if Self::is_empty_or_airlike((state.position.0 + dx, state.position.1), grid, behaviors_grid) {
            return Some((state.position.0 + dx, state.position.1));
        }

        None
    }
}