use macroquad::{color, texture::{draw_texture, Texture2D}, time};

use crate::utils::paths::{self, AssetType};

pub struct Bird {
    animation_timer: f32,

    sprites: Vec<Texture2D>,
    reverse: bool,
    active_sprite_idx: usize
}

impl Bird {
    pub async fn new() -> Self {
        Self {
            animation_timer: 0.0,
            sprites: vec![
                paths::get_asset("yellowbird-upflap.png", AssetType::Sprite).await.unwrap(),
                paths::get_asset("yellowbird-midflap.png", AssetType::Sprite).await.unwrap(),
                paths::get_asset("yellowbird-downflap.png", AssetType::Sprite).await.unwrap() 
            ],
            reverse: false,
            active_sprite_idx: 0
        }
    }

    pub fn update(&mut self) {
        self.animation_timer += time::get_frame_time();
        if self.animation_timer >= 0.15 {
            self.animation_timer = 0.0;

            if self.reverse { 
                self.active_sprite_idx -= 1 
            } else { 
                self.active_sprite_idx += 1 
            }

            if self.active_sprite_idx >= self.sprites.len() - 1 || self.active_sprite_idx <= 0 {
                self.reverse = !self.reverse;
            }
        }
    }

    pub fn draw(&self) {
        draw_texture(&self.sprites[self.active_sprite_idx], 10., 10., color::WHITE)
    }
}