use macroquad::{color::{Color, WHITE}, input::{self, MouseButton}, texture::{draw_texture, Texture2D}, time};
use rand::rngs::ThreadRng;
use crate::{entities::{bird::Bird, entity::Entity, ground::Ground, pipes::Pipes}, utils::paths};
use super::state::State;

const PIPE_SPAWN: f32 = 1.5;

pub struct Game {
    entities: Vec<Box<dyn Entity>>,

    rng: ThreadRng,

    pipes: Vec<Pipes>,
    pipe_timer: f32,
    pipe: Pipes,

    start_texture: Texture2D,
    started: bool,
    alpha: f32,

    done: bool
}

impl Game {
    pub async fn new() -> Game {
        Game {
            entities: vec![
                Box::new(Ground::new().await),
                Box::new(Bird::new().await),
            ],

            rng: rand::thread_rng(),

            pipes: Vec::new(),
            pipe_timer: 0.0,
            pipe: Pipes::new().await,

            start_texture: paths::get_asset("message.png").await.unwrap(),
            started: false,
            alpha: 0.0,
            done: false
        }
    }
}

impl State for Game {
    fn init(&mut self) {
        for e in &mut self.entities {
            e.init();
        }
    }

    fn update(&mut self) {
        if !self.started && input::is_mouse_button_pressed(MouseButton::Left) {
            self.started = true;
        }

        for e in &mut self.entities {
            e.update(!self.started);
        }

        let pipes = &mut self.pipes;
        pipes.retain_mut(|pipe| {
            pipe.update(!self.started);

            // Remove if pipe reaches sprite width - 5
            pipe.x() > -pipe.width() - 5.0
        });

        if self.started {
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
            if self.alpha <= 1.0 { self.alpha += 0.1; }
            return;
        }

        if self.alpha >= 0.0 { self.alpha -= 0.1; }
        if self.alpha >= 1.0 { self.done = true; }
    }

    fn draw(&self) {
        for pipe in &self.pipes {
            pipe.draw();
        }

        for e in &self.entities {
            e.draw();
        }

        if !self.done {
            draw_texture(&self.start_texture, 50., 40., Color { a: self.alpha, ..WHITE });
        }
    }
}