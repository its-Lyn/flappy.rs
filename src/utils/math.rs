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

pub struct DigitsIter(u32, bool);

impl DigitsIter {
    pub fn new(num: u32) -> Self {
        Self(num, false)
    }
}

impl Iterator for DigitsIter {
    type Item = u8;

    fn next(&mut self) -> Option<u8> {
        if self.0 > 0 || !self.1 {
            let d = (self.0 % 10) as u8;
            self.0 /= 10;
            self.1 = true;
            Some(d)
        } else {
            None
        }
    }
}