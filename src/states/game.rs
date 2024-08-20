use macroquad::{color::{Color, WHITE}, input::{self, MouseButton}, texture::{draw_texture, Texture2D}};

use crate::{entities::{bird::Bird, entity::Entity, ground::Ground}, utils::paths};

use super::state::State;

pub struct Game {
    entities: Vec<Box<dyn Entity>>,

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
                Box::new(Bird::new().await)
            ],

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
            e.update(self.started);
        }

        if !self.started {
            if self.alpha <= 1.0 { self.alpha += 0.1; }
            return;
        }

        if self.alpha >= 0.0 { self.alpha -= 0.1; }
        if self.alpha >= 1.0 { self.done = true; }
    }

    fn draw(&self) {
        for e in &self.entities {
            e.draw();
        }

        if !self.done {
            draw_texture(&self.start_texture, 50., 40., Color { a: self.alpha, ..WHITE });
        }
    }
}