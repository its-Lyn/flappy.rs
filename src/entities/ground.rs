use macroquad::{color, math::{Rect, Vec2}, shapes::draw_rectangle_lines, texture::{draw_texture, Texture2D}};

use crate::{utils::paths, DEV_MODE, GAME_HEIGHT, GAME_WIDTH};

use super::entity::Entity;

const SPEED: f32 = 2.0;

pub struct Ground {
    sprite: Texture2D,

    pos: Vec2,
    second_pos: Vec2,

    collider: Rect
}

impl Ground {
    pub async fn new() -> Self {
        Ground {
            sprite: paths::get_asset("base.png").await.unwrap(),

            pos: Vec2::ZERO,
            second_pos: Vec2::ZERO,

            collider: Rect::new(0.0, 0.0, 0.0, 0.0)
        }
    }

    fn update_collider(&mut self) {
        self.collider.x = 0.0;
        self.collider.y = GAME_HEIGHT - self.sprite.height();

        self.collider.w = GAME_WIDTH;
        self.collider.h = self.sprite.height();
    }

    pub fn get_collider(&self) -> Rect {
        self.collider
    }
}

impl Entity for Ground {
    fn init(&mut self) {
        self.pos.y = GAME_HEIGHT - self.sprite.height();

        self.second_pos.y = GAME_HEIGHT - self.sprite.height();
        self.second_pos.x = self.sprite.width();
    }

    fn update(&mut self, paused: bool) {
        if paused { return; }

        self.update_collider();
        
        self.pos.x -= SPEED;
        if self.pos.x < 0. - self.sprite.width() {
            self.pos.x = GAME_WIDTH;
        }

        self.second_pos.x -= SPEED;
        if self.second_pos.x < 0. - self.sprite.width() {
            self.second_pos.x = GAME_WIDTH;
        }
    }

    fn draw(&self) {
        // Draw the first ground
        draw_texture(&self.sprite, self.pos.x, self.pos.y, color::WHITE);

        // Draw the second ground
        draw_texture(&self.sprite, self.second_pos.x, self.second_pos.y, color::WHITE);

        if DEV_MODE {
            draw_rectangle_lines(self.collider.x, self.collider.y, self.collider.w, self.collider.h, 5.0, color::RED);
        }
    }
}