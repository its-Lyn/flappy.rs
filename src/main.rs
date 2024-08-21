use macroquad::{camera::{set_camera, set_default_camera, Camera2D}, color, math::{vec2, Rect}, texture::{draw_texture, draw_texture_ex, render_target, DrawTextureParams, FilterMode}, window::{clear_background, next_frame, screen_height, screen_width, Conf}};
use states::{game::Game, state::State};
use utils::paths;

mod utils;
mod entities;
mod states;

pub const DEV_MODE: bool = false;

pub const GAME_WIDTH: f32 = 288.0;
pub const GAME_HEIGHT: f32 = 512.0;

fn make_conf() -> Conf {
    Conf {
        window_title: "Flappy Bird".to_string(),
        window_resizable: true,
        window_width: 288,
        window_height: 512,
        ..Default::default()
    }
}

#[macroquad::main(make_conf)]
async fn main() {
    // Init
    let renderer = render_target(GAME_WIDTH as u32, GAME_HEIGHT as u32);
    renderer.texture.set_filter(FilterMode::Nearest);

    let mut renderer_camera = Camera2D::from_display_rect(Rect::new(0., 0., GAME_WIDTH, GAME_HEIGHT));
    renderer_camera.render_target = Some(renderer.clone());

    // Get backfround now as we won't be touching it
    let background = paths::get_asset("background-day.png").await.unwrap();

    let mut main_state: Box<dyn State>;
    main_state = Box::new(Game::new().await);
    main_state.init();

    loop {
        // Update
        let scale: f32 = f32::min(
            screen_width() / GAME_WIDTH,
            screen_height() / GAME_HEIGHT
        );

        main_state.update();

        // Draw
        clear_background(color::BLACK);

        // Drawing inside the game world
        set_camera(&renderer_camera);

        clear_background(color::WHITE);
        draw_texture(&background, 0., 0., color::WHITE);

        main_state.draw();

        // Drawing on the whole screen
        set_default_camera();

        clear_background(color::BLACK);
        draw_texture_ex(
            &renderer.texture,
            (screen_width() - (GAME_WIDTH * scale)) * 0.5,
            (screen_height() - (GAME_HEIGHT * scale)) * 0.5,
            color::WHITE,
            DrawTextureParams {
                dest_size: Some(vec2(GAME_WIDTH * scale, GAME_HEIGHT * scale)),
                flip_y: true,
                ..Default::default()
            }
        );

        next_frame().await
    }
}
