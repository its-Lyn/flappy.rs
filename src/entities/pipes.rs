use macroquad::{color, math::Vec2, texture::{draw_texture, draw_texture_ex, DrawTextureParams, Texture2D}};
use rand::{rngs::ThreadRng, Rng};
use crate::{utils::paths, GAME_HEIGHT, GAME_WIDTH};
use super::entity::Entity;

const SPAWN_OFFSET: f32 = 20.0;
const SPEED: f32 = 2.0;

const PIPE_GAP: f32 = 90.0;
const PIPE_MIN_OFFSET: f32 = 180.0;
const PIPE_MAX_OFFSET: f32 = 120.0;

#[derive(Clone)]
pub struct Pipes {
    sprite: Texture2D,

    first_pos: Vec2,
    second_pos: Vec2
}

impl Pipes {
    pub async fn new() -> Self {
        Pipes {
            sprite: paths::get_asset("pipe-green.png").await.unwrap(),

            first_pos: Vec2::ZERO,
            second_pos: Vec2::ZERO
        }
    }

    pub fn set_y(&mut self, rng: &mut ThreadRng) {
        let min_spawn: f32 = GAME_HEIGHT - PIPE_MIN_OFFSET;
        let max_spawn: f32 = PIPE_MAX_OFFSET;

        let y = rng.gen_range(max_spawn..=min_spawn);

        self.first_pos.y = y;
        self.second_pos.y = y - self.sprite.height() - PIPE_GAP; 
    }

    pub fn x(&self) -> f32 {
        self.first_pos.x
    }

    pub fn width(&self) -> f32 {
        self.sprite.width()
    }
}

impl Entity for Pipes {
    fn init(&mut self) {
        self.first_pos.x = GAME_WIDTH + SPAWN_OFFSET;
        self.second_pos.x = GAME_WIDTH + SPAWN_OFFSET;
    }

    fn update(&mut self, paused: bool) {
        if paused { return; }

        self.first_pos.x -= SPEED;
        self.second_pos.x -= SPEED;
    }

    fn draw(&self) {
        draw_texture(&self.sprite, self.first_pos.x, self.first_pos.y, color::WHITE);

        draw_texture_ex(
            &self.sprite,
            self.second_pos.x,
            self.second_pos.y,
            color::WHITE,
            DrawTextureParams {
                flip_y: true,
                ..Default::default()
            }
        );
    }
}