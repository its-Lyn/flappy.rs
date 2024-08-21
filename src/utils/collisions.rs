use macroquad::math::Rect;

pub fn rect_collision(r1: &Rect, r2: &Rect) -> bool {
    if r1.x < r2.x + r2.w && r2.x < r1.x + r1.w
    && r1.y < r2.y + r2.h && r2.y < r1.y + r1.h {
        return true;
    }

    false
}