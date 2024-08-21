use macroquad::{audio::{self, Sound}, color, input::{self, MouseButton}, math::Vec2, texture::{draw_texture_ex, DrawTextureParams, Texture2D}, time};
use crate::{utils::{math::{deg_to_rad, move_towards}, paths}, GAME_HEIGHT};

use super::entity::Entity;

const GRAVITY: f32 = 0.50;
const TERMINAL_VELOCITY: f32 = 10.50;
const JUMP_VELOCITY: f32 = -6.0;

const MAX_DOWNWARD_ANGLE: f32 = 40.0;
const MAX_UPWARD_ANGLE: f32 = -20.0;

const FLAPPING_SPEED: f32 = 0.05;
const FALLING_SPEED: f32 = 0.15;

pub struct Bird {
    animation_timer: f32,
    animation_speed: f32,

    sprites: Vec<Texture2D>,
    reverse: bool,
    active_sprite_idx: usize,

    rotation: f32,
    rotation_speed: f32,
    rotation_angle: f32,

    pos: Vec2,
    vel: Vec2,

    flap_sound: Sound,
}

impl Bird {
    pub async fn new() -> Self {
        Self {
            animation_timer: 0.0,
            animation_speed: 0.15,
            sprites: vec![
                paths::get_asset("yellowbird-upflap.png").await.unwrap(),
                paths::get_asset("yellowbird-midflap.png").await.unwrap(),
                paths::get_asset("yellowbird-downflap.png").await.unwrap() 
            ],

            rotation: 0.0,
            rotation_speed: 8.0,
            rotation_angle: MAX_UPWARD_ANGLE,

            reverse: false,
            active_sprite_idx: 0,

            pos: Vec2::ZERO,
            vel: Vec2::ZERO,

            flap_sound: paths::get_audio("wing.wav").await.unwrap(),
        }
    }

    fn cycle_animation(&mut self) {
        self.animation_timer += time::get_frame_time();
        if self.animation_timer >= self.animation_speed {
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
}

impl Entity for Bird {
    fn init(&mut self) {
        let current_texture: &Texture2D = &self.sprites[self.active_sprite_idx];
        
        self.pos.y = (GAME_HEIGHT / 2.0) - current_texture.width();
        self.pos.x = 70.0;
    }

    fn update(&mut self, paused: bool) {
        self.cycle_animation();

        if !paused {
            return;
        }

        // Apply gravity
        self.vel.y = move_towards(self.vel.y, TERMINAL_VELOCITY, GRAVITY);

        // Rotate the bird every frame
        self.rotation = move_towards(self.rotation, self.rotation_angle, self.rotation_speed);

        // Flap
        if input::is_mouse_button_pressed(MouseButton::Left) {
            audio::play_sound_once(&self.flap_sound);

            self.rotation_angle = MAX_UPWARD_ANGLE;

            self.vel.y = JUMP_VELOCITY;
            self.animation_speed = FLAPPING_SPEED;
        }

        // Bonk
        if self.pos.y <= -self.sprites[self.active_sprite_idx].height() * 2.0 {
            self.pos.y = -self.sprites[self.active_sprite_idx].height() * 1.9;
            self.vel.y = 0.0;
        }

        // Speed up the cycle animation
        // When the velocity reaches 1.5
        // This surprisingly works well for some reason
        if self.vel.y > 1.5 {
            self.rotation_angle = MAX_DOWNWARD_ANGLE;
            self.animation_speed = FALLING_SPEED;
        }

        self.pos += self.vel;
    }

    fn draw(&self) {
        draw_texture_ex(
            &self.sprites[self.active_sprite_idx],
            self.pos.x,
            self.pos.y,
            color::WHITE,
            DrawTextureParams {
                rotation: deg_to_rad(self.rotation),
                ..Default::default()
            }
        );
    }
}