use macroquad::{audio::{self, Sound}, color::{Color, WHITE}, input::{self, MouseButton}, texture::{draw_texture, Texture2D}, time};
use rand::rngs::ThreadRng;
use crate::{entities::{bird::Bird, entity::Entity, ground::Ground, pipes::Pipes}, utils::{collisions, math::DigitsIter, paths}, GAME_WIDTH};
use super::state::State;

const PIPE_SPAWN: f32 = 1.5;

pub struct Game {
    bird: Bird,
    ground: Ground,

    score: i32,
    score_alpha: f32,
    score_changed: bool,
    score_textured: Vec<Texture2D>,
    score_assets: Vec<Texture2D>,
    score_sound: Sound,

    rng: ThreadRng,

    pipes: Vec<Pipes>,
    pipe_timer: f32,
    pipe: Pipes,

    death_sound: Sound,
    death_fall_sound: Sound,

    playing: bool,
    falling: bool,

    start_texture: Texture2D,
    started: bool,
    start_alpha: f32,
    done: bool,
}

impl Game {
    pub async fn new() -> Game {
        Game {
            bird: Bird::new().await,
            ground: Ground::new().await,

            score: 0,
            score_alpha: 0.0,
            score_changed: false,
            score_textured: Vec::new(),
            score_assets: vec![
                paths::get_asset("0.png").await.unwrap(),
                paths::get_asset("1.png").await.unwrap(),
                paths::get_asset("2.png").await.unwrap(),
                paths::get_asset("3.png").await.unwrap(),
                paths::get_asset("4.png").await.unwrap(),
                paths::get_asset("5.png").await.unwrap(),
                paths::get_asset("6.png").await.unwrap(),
                paths::get_asset("7.png").await.unwrap(),
                paths::get_asset("8.png").await.unwrap(),
                paths::get_asset("9.png").await.unwrap()
            ],
            score_sound: paths::get_audio("point.wav").await.unwrap(),

            rng: rand::thread_rng(),

            pipes: Vec::new(),
            pipe_timer: 0.0,
            pipe: Pipes::new().await,

            death_sound: paths::get_audio("hit.wav").await.unwrap(),
            death_fall_sound: paths::get_audio("die.wav").await.unwrap(),
            
            playing: true,
            falling: false,

            start_texture: paths::get_asset("message.png").await.unwrap(),
            started: false,
            start_alpha: 0.0,
            done: false,
        }
    }

    fn update_score(&mut self) {
        self.score_textured.clear();
        for digit in DigitsIter::new(self.score as u32) {
            self.score_textured.push(self.score_assets[digit as usize].clone())
        }
    }
}

impl State for Game {
    fn init(&mut self) {
        self.bird.init();
        self.ground.init();      

        self.score_textured.push(self.score_assets[0].clone());  
    }

    fn update(&mut self) {
        if !self.started && input::is_mouse_button_pressed(MouseButton::Left) {
            self.started = true;
            self.playing = false;
        }

        if !self.playing && collisions::rect_collision(&self.bird.get_collider(), &self.ground.get_collider()) {
            self.playing = true;
            self.bird.hard_pause = true;

            audio::play_sound_once(&self.death_sound);
        }

        self.bird.update(self.playing);
        self.ground.update(self.playing);

        if self.falling {
            self.bird.die();

            if collisions::rect_collision(&self.bird.get_collider(), &self.ground.get_collider()) {
                self.falling = false;
            }
        }
 
        let pipes = &mut self.pipes;
        pipes.retain_mut(|pipe| {
            pipe.update(self.playing);

            // Death check
            if (collisions::rect_collision(&self.bird.get_collider(), &pipe.get_colliders().0) 
            || collisions::rect_collision(&self.bird.get_collider(), &pipe.get_colliders().1))
            && !pipe.touched_death {
                self.playing = true;
                self.falling = true;

                audio::play_sound_once(&self.death_sound);
                audio::play_sound_once(&self.death_fall_sound);

                pipe.touched_death = true;
            }

            // Score check
            if collisions::rect_collision(&self.bird.get_collider(), &pipe.get_score_collider()) && !pipe.touched {
                self.score += 1;
                self.score_changed = true;
                audio::play_sound_once(&self.score_sound);

                pipe.touched = true;
            }

            // Remove if pipe reaches sprite width - 5
            pipe.x() > -pipe.width() - 5.0
        });

        if self.score_changed {
            self.update_score();
            self.score_changed = false;
        }

        if self.started {
            if self.score_alpha <= 1.0 && self.score > 0 {
                self.score_alpha += 0.2;
            }

            self.pipe_timer += time::get_frame_time();
            if self.pipe_timer >= PIPE_SPAWN {
                self.pipe_timer = 0.0;

                let mut new_pipe: Pipes = self.pipe.clone();
                new_pipe.init();
                new_pipe.set_y(&mut self.rng);

                self.pipes.push(new_pipe);
            }
        }

        if !self.started {
            if self.start_alpha <= 1.0 { self.start_alpha += 0.1; }
            return;
        }

        if self.start_alpha >= 0.0 { self.start_alpha -= 0.1; }
        if self.start_alpha >= 1.0 { self.done = true; }
    }

    fn draw(&self) {
        for pipe in &self.pipes {
            pipe.draw();
        }

        self.bird.draw();
        self.ground.draw();

        if !self.done {
            draw_texture(&self.start_texture, 50., 40., Color { a: self.start_alpha, ..WHITE });
        }

        // Draw score
        if self.score == 0 || self.playing {
            return;
        }

        let mut base_position = (GAME_WIDTH / 2.0) + ((self.score_assets[1].width() / 2.0) * self.score_textured.len() as f32);
        for score_texture in &self.score_textured {
            base_position -= score_texture.width();

            draw_texture(score_texture, base_position, 10.0, Color { a: self.score_alpha, ..WHITE });
        }
    }
}