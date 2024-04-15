use crate::sandsim::behaviors::*;

pub struct MoveDown {
    acceleration: f64,
    max_velocity: f64,
    velocity: f64,

    float_position: FloatPosition,
    integer_position: Position,
}

impl Behavior for MoveDown {
    fn get_id(&self) -> BehaviorId {
        MOVE_DOWN_ID
    }

    fn update(&mut self, state: &ParticleState, dt: f64, grid: &mut Vec<Vec<ParticleId>>, behaviors_grid: &mut Vec<Vec<BehaviorId>>) -> Vec<ParticleAction> {
        // Check if we have changed position between two frames
        if self.integer_position != state.position {
            self.integer_position = state.position;
            // self.stop_motion();
            // Should return there ?
        }

        // Regular update
        let sign = self.acceleration.signum();
        self.velocity += self.acceleration * dt;
        if self.velocity.abs() > self.max_velocity {
            self.velocity = self.max_velocity * sign;
        }
        self.float_position.1 += self.velocity * dt;

        // Check if we changed grid cell 
        let mut new_position = self.to_integer_position();
        if new_position == self.integer_position {
            return vec![]; // We did not move (or moved within the same cell)
        }

        // We moved to a new cell, check if its in bounds
        if new_position.1 < 0 || new_position.1 >= grid.len() as i32 { // Only check vertical axis because can only fall down
            self.stop_motion();
            return vec![]; // We are out of bounds
        }

        // We are in bounds, find target empty cell
        if let Some(dx) = self.find_empty_cell(state.position, new_position.1 - state.position.1, &grid, &behaviors_grid) {
            new_position.0 += dx;
            
            // Swap particle IDs
            grid[new_position.1 as usize][new_position.0 as usize] = grid[self.integer_position.1 as usize][self.integer_position.0 as usize];
            grid[self.integer_position.1 as usize][self.integer_position.0 as usize] = EMPTY_ID;

            // Swap behaviors IDs
            let tmp = behaviors_grid[new_position.1 as usize][new_position.0 as usize];
            behaviors_grid[new_position.1 as usize][new_position.0 as usize] = grid[self.integer_position.1 as usize][self.integer_position.0 as usize];
            behaviors_grid[self.integer_position.1 as usize][self.integer_position.0 as usize] = tmp;
            
            self.integer_position = new_position;
            self.float_position.0 += dx as f64;

            vec![ParticleAction::SetPosition { position: new_position }]
        } else {
            // No available cell, stop
            self.stop_motion();
            vec![]
        }
    }
}

impl MoveDown {
    pub fn boxed(position: Position, max_velocity: f64, acceleration: f64) -> Box<dyn Behavior> {
        let float_position = (position.0 as f64, position.1 as f64);
        Box::new(Self { float_position, integer_position: position, max_velocity: max_velocity.abs(), acceleration, velocity: 0.})
    }

    fn to_integer_position(&self) -> Position {
        (self.float_position.0.round() as i32, self.float_position.1.round() as i32)
    }

    fn stop_motion(&mut self) {
        self.velocity = 0.;
        self.float_position.1 = self.integer_position.1 as f64;
    }

    fn find_empty_cell(&self, (x, y): Position, dy: i32, grid: &Vec<Vec<ParticleId>>, behaviors_grid: &Vec<Vec<BehaviorId>>) -> Option<i32> {
        let self_air_like = has_behavior((x, y), behaviors_grid, AIR_LIKE_ID);

        // Check (x, y) first 
        if (!self_air_like && has_behavior((x, y + dy), behaviors_grid, AIR_LIKE_ID)) || (self_air_like && grid[(y + dy) as usize][x as usize] == EMPTY_ID) {
            return Some(0);
        }

        // Otherwise, check both side, first choosen randomly
        let dx = if rand::random::<f32>() < 0.5 { 1 } else { -1 };
        let width = behaviors_grid[0].len() as i32;
        if x + dx >= 0 && x + dx < width && ((!self_air_like && has_behavior((x + dx, y + dy), behaviors_grid, AIR_LIKE_ID)) || (self_air_like && grid[(y + dy) as usize][(x + dx) as usize] == EMPTY_ID)) {
            return Some(dx);
        }

        if x - dx >= 0 && x - dx < width && ((!self_air_like && has_behavior((x + dx, y + dy), behaviors_grid, AIR_LIKE_ID)) || (self_air_like && grid[(y + dy) as usize][(x + dx) as usize] == EMPTY_ID)) {
            return Some(-dx);
        }

        None
    }
}
