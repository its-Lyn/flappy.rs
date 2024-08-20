use crate::entities::{bird::Bird, entity::Entity, ground::Ground};

use super::state::State;

pub struct Game {
    entities: Vec<Box<dyn Entity>>
}

impl Game {
    pub async fn new() -> Game {
        Game {
            entities: vec![
                Box::new(Ground::new().await),
                Box::new(Bird::new().await)
            ]
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
        for e in &mut self.entities {
            e.update();
        }
    }

    fn draw(&self) {
        for e in &self.entities {
            e.draw();
        }
    }
}