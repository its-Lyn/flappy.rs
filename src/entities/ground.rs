use macroquad::{color, math::Vec2, texture::{draw_texture, Texture2D}};

use crate::{utils::paths, GAME_HEIGHT, GAME_WIDTH};

pub struct Ground {
    sprite: Texture2D,

    pos: Vec2,
    second_pos: Vec2
}

impl Ground {
    pub async fn new() -> Self {
        Ground {
            sprite: paths::get_asset("base.png").await.unwrap(),

            pos: Vec2::ZERO,
            second_pos: Vec2::ZERO
        }
    }

    pub fn init(&mut self) {
        self.pos.y = GAME_HEIGHT - self.sprite.height();

        self.second_pos.y = GAME_HEIGHT - self.sprite.height();
        self.second_pos.x = self.sprite.width();
    }


    pub fn update(&mut self) {
        self.pos.x -= 2.0;
        if self.pos.x < 0. - self.sprite.width() {
            self.pos.x = GAME_WIDTH;
        }

        self.second_pos.x -= 2.0;
        if self.second_pos.x < 0. - self.sprite.width() {
            self.second_pos.x = GAME_WIDTH;
        }
    }

    pub fn draw(&self) {
        // Draw the first ground
        draw_texture(&self.sprite, self.pos.x, self.pos.y, color::WHITE);

        // Draw the second ground
        draw_texture(&self.sprite, self.second_pos.x, self.second_pos.y, color::WHITE);
    }
}