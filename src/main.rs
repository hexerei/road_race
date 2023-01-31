use rusty_engine::prelude::*;

struct GameState {
    health_amount: u8,
    lost: bool,
}

fn main() {
    let mut game = Game::new();

    // game setup goes here
    game.window_settings(WindowDescriptor {
        title: "Road Racer".into(),
        width: 800.0,
        height: 300.0,
        ..Default::default()
    });

    // create player sprite
    let player1 = game.add_sprite("player1", SpritePreset::RacingCarBlue);
    player1.translation.x = -300.0;
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
    // game logic goes here
}