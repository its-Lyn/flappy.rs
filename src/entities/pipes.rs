use macroquad::{color, math::{Rect, Vec2}, shapes::draw_rectangle_lines, texture::{draw_texture, draw_texture_ex, DrawTextureParams, Texture2D}};
use rand::{rngs::ThreadRng, Rng};
use crate::{utils::paths, DEV_MODE, GAME_HEIGHT, GAME_WIDTH};
use super::entity::Entity;

const SPAWN_OFFSET: f32 = 20.0;
const SPEED: f32 = 2.0;

const PIPE_GAP: f32 = 80.0;
const PIPE_MIN_OFFSET: f32 = 180.0;
const PIPE_MAX_OFFSET: f32 = 120.0;

const SCORE_SIZE: f32 = 20.0;

#[derive(Clone)]
pub struct Pipes {
    sprite: Texture2D,

    first_pos: Vec2,
    second_pos: Vec2,

    first_collider: Rect,
    second_collider: Rect,

    score_collider: Rect,

    pub touched: bool,
    pub touched_death: bool,
}

impl Pipes {
    pub async fn new() -> Self {
        Pipes {
            sprite: paths::get_asset("pipe-green.png").await.unwrap(),

            first_pos: Vec2::ZERO,
            second_pos: Vec2::ZERO,

            first_collider: Rect::new(0.0, 0.0, 0.0, 0.0),
            second_collider: Rect::new(0.0, 0.0, 0.0, 0.0),

            score_collider: Rect::new(0.0, 0.0, 0.0, 0.0),

            touched: false,
            touched_death: false,
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

    fn update_colliders(&mut self) {
        // Bottom pipe
        self.first_collider.x = self.first_pos.x;
        self.first_collider.y = self.first_pos.y;

        self.first_collider.w = self.sprite.width();
        self.first_collider.h = self.sprite.height();

        // Top pipe
        self.second_collider.x = self.second_pos.x;
        self.second_collider.y = self.second_pos.y;

        self.second_collider.w = self.sprite.width();
        self.second_collider.h = self.sprite.height();

        // Score
        self.score_collider.x = self.first_pos.x + (self.sprite.width() / 2.0) - (SCORE_SIZE / 2.0);
        self.score_collider.y = self.first_pos.y - PIPE_GAP;

        self.score_collider.w = SCORE_SIZE;
        self.score_collider.h = PIPE_GAP;
    }

    pub fn get_colliders(&self) -> (Rect, Rect) {
        (self.first_collider, self.second_collider)
    }

    pub fn get_score_collider(&self) -> Rect {
        self.score_collider
    }

}

impl Entity for Pipes {
    fn init(&mut self) {
        self.first_pos.x = GAME_WIDTH + SPAWN_OFFSET;
        self.second_pos.x = GAME_WIDTH + SPAWN_OFFSET;
    }

    fn update(&mut self, paused: bool) {
        if paused { return; }

        self.update_colliders();

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

        if DEV_MODE {
            draw_rectangle_lines(self.first_collider.x, self.first_collider.y, self.first_collider.w, self.first_collider.h, 5.0, color::RED);
            draw_rectangle_lines(self.second_collider.x, self.second_collider.y, self.second_collider.w, self.second_collider.h, 5.0, color::RED);
            draw_rectangle_lines(self.score_collider.x, self.score_collider.y, self.score_collider.w, self.score_collider.h, 5.0, color::WHITE);
        }
   }
}