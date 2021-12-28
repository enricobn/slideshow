pub trait Velocity {
    /// perc must be between 0. and 1.0
    fn get_velocity(&self, perc: f32) -> f32;
}

pub struct StepsVelocity {
    steps: Vec<f32>
}

impl StepsVelocity {
    /// velocity is interpolated (linearly) from the given velocities
    pub fn new(steps: Vec<f32>) -> StepsVelocity {
        StepsVelocity { steps }
    }
}

impl Velocity for StepsVelocity {
    fn get_velocity(&self, perc: f32) -> f32 {
        let step_width = 1.0 / (self.steps.len() - 1) as f32;
        let steps = perc / step_width;

        // perc is between left and right steps 
        let left_step = steps.floor();
        let right_step = steps.ceil();
        let left_velocity = self.steps[left_step as usize];
        let right_velocity = self.steps[right_step as usize];

        let distance_from_right_step = step_width * right_step - perc;

        let velocity_difference = left_velocity - right_velocity;
        let velocity_relative_to_right_step = velocity_difference * distance_from_right_step / step_width;

        let v = velocity_relative_to_right_step + right_velocity;

        /*
        println!("*****************************************", );
        println!("step width={} step_height={}", step_width, step_height);
        println!("perc={}", perc);
        println!("left={} right={}", left, right);
        println!("distance_from_left={} distance_from_right={}", distance_from_left, distance_from_right);
        println!("v={}", v);
        */

        v
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_steps_velocity() {
        let vel = StepsVelocity::new(vec![1.0, 0.75, 0.5, 0.25, 0.0]); // 0 0.25 0.5 0.75 1

        let mut v = vel.get_velocity(0.1);
        assert_eq!(true, v > 0.75 && v < 1.0);

        v = vel.get_velocity(0.25);
        assert_eq!(0.75, v);

        v = vel.get_velocity(0.3);
        assert_eq!(true, v > 0.5 && v < 0.75);
    }
}
