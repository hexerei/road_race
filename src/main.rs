use rusty_engine::prelude::*;

const PLAYER_SPEED: f32 = 250.0; // pixel per seconds
const WINDOW_WIDTH: f32 = 800.0;
const WINDOW_HEIGHT: f32 = 300.0;

struct GameState {
    health_amount: u8,
    lost: bool,
}

fn main() {
    let mut game = Game::new();

    // game setup goes here
    game.window_settings(WindowDescriptor {
        title: "Road Racer".into(),
        width: WINDOW_WIDTH,
        height: WINDOW_HEIGHT,
        ..Default::default()
    });

    // create player sprite
    let player1 = game.add_sprite("player1", SpritePreset::RacingCarBlue);
    player1.translation.x = -(WINDOW_WIDTH / 2.0 - 100.0);
    player1.layer = 10.0;
    player1.collision = true;

    // start some background music
    game.audio_manager.play_music(MusicPreset::WhimsicalPopsicle, 0.2);


    game.add_logic(game_logic);
    game.run(GameState {
        health_amount: 5,
        lost: false,
    });
}

fn game_logic(engine: &mut Engine, game_state: &mut GameState) {
    // Collect keyboard input
    let mut direction = 0.0;
    if engine.keyboard_state.pressed(KeyCode::Left) {
        direction += 1.0;
    }
    if engine.keyboard_state.pressed(KeyCode::Right) {
        direction -= 1.0;
    }

    // Move the player sprite
    let player1 = engine.sprites.get_mut("player1").unwrap();
    player1.translation.y += direction * PLAYER_SPEED * engine.delta_f32;
    player1.rotation = direction * 0.15;
    if player1.translation.y < -(WINDOW_HEIGHT / 2.0) || player1.translation.y > WINDOW_HEIGHT / 2.0 {
        game_state.health_amount = 0;
    }
}