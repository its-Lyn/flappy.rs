use macroquad::{color, input::{self, MouseButton}, math::Vec2, texture::{draw_texture, Texture2D}};

use crate::{utils::paths, GAME_HEIGHT, GAME_WIDTH};

use super::entity::Entity;

pub struct Ground {
    sprite: Texture2D,

    pos: Vec2,
    second_pos: Vec2,

    paused: bool
}

impl Ground {
    pub async fn new() -> Self {
        Ground {
            sprite: paths::get_asset("base.png").await.unwrap(),

            pos: Vec2::ZERO,
            second_pos: Vec2::ZERO,

            paused: true
        }
    }
}

impl Entity for Ground {
    fn init(&mut self) {
        self.pos.y = GAME_HEIGHT - self.sprite.height();

        self.second_pos.y = GAME_HEIGHT - self.sprite.height();
        self.second_pos.x = self.sprite.width();
    }

    fn update(&mut self) {
        if self.paused {
            if input::is_mouse_button_pressed(MouseButton::Left) {
                self.paused = false;
            } else {
                return;
            }
        }
        
        self.pos.x -= 2.0;
        if self.pos.x < 0. - self.sprite.width() {
            self.pos.x = GAME_WIDTH;
        }

        self.second_pos.x -= 2.0;
        if self.second_pos.x < 0. - self.sprite.width() {
            self.second_pos.x = GAME_WIDTH;
        }
    }

    fn draw(&self) {
        // Draw the first ground
        draw_texture(&self.sprite, self.pos.x, self.pos.y, color::WHITE);

        // Draw the second ground
        draw_texture(&self.sprite, self.second_pos.x, self.second_pos.y, color::WHITE);
    }
}