pub trait Velocity {

    fn getVelocity(&self, perc: f32) -> f32;

}

pub struct StepsVelocity {
    steps: Vec<f32>
}

impl StepsVelocity {

    pub fn new(steps: Vec<f32>) -> StepsVelocity {
        StepsVelocity{steps: steps}
    }

}

impl Velocity for StepsVelocity {

    fn getVelocity(&self, perc: f32) -> f32 {
        let step_width = 1.0 / (self.steps.len() - 1) as f32;
        let steps = perc / step_width;

        let left_step = steps.floor();
        let right_step = steps.ceil();
        let left = self.steps[left_step as usize];
        let right = self.steps[right_step as usize];

        let distance_from_right = step_width * right_step - perc;

        let step_height = left - right;
        let h = step_height * distance_from_right / step_width;

        let v = h + right;

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

#[test]
fn test_steps_velocity() {
    let vel = StepsVelocity::new(vec![1.0, 0.75, 0.5, 0.25, 0.0]); // 0 0.25 0.5 0.75 1

    let mut v = vel.getVelocity(0.1);
    assert_eq!(true, v > 0.75 && v < 1.0);

    v = vel.getVelocity(0.25);
    assert_eq!(0.75, v);

    v = vel.getVelocity(0.3);
    assert_eq!(true, v > 0.5 && v < 0.75);
}
