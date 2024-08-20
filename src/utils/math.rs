use std::f32::consts::PI;

pub fn move_towards(from: f32, to: f32, delta: f32) -> f32 {
    if f32::abs(to - from) <= delta {
        return to;
    }

    from + (f32::signum(to - from) * delta)
}

pub fn deg_to_rad(degrees: f32) -> f32 {
    degrees * PI / 180.0
}