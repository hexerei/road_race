use rusty_engine::prelude::*;
use rand::prelude::*;

const WINDOW_WIDTH: f32 = 800.0;
const WINDOW_HEIGHT: f32 = 300.0;
const CENTER_X: f32 = WINDOW_WIDTH / 2.0;
const CENTER_Y: f32 = WINDOW_HEIGHT / 2.0;

const PLAYER_SPEED: f32 = 250.0; // pixel per seconds

const ROAD_SPEED: f32 = 400.0;
const ROAD_SPACING: f32 = 100.0;

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
    player1.translation.x = -(CENTER_X - 100.0);
    player1.layer = 10.0;
    player1.collision = true;

    // Create the road lines
    for i in 0..10 {
        let roadline = game.add_sprite(format!("roadline{}", i), SpritePreset::RacingBarrierWhite);
        roadline.scale = 0.1;
        roadline.translation.x = -(CENTER_X - 40.0) + ROAD_SPACING * i as f32;
    }

    // Create the obstacles
    let obstacle_presets = vec![
        SpritePreset::RacingBarrelBlue,
        SpritePreset::RacingBarrelRed,
        SpritePreset::RacingConeStraight
    ];
    for (i, preset) in obstacle_presets.into_iter().enumerate() {
        let obstacle = game.add_sprite(format!("obstacle{}", i), preset);
        obstacle.layer = 5.0;
        obstacle.collision = true;
        obstacle.translation.x = thread_rng().gen_range(CENTER_X..(WINDOW_WIDTH + 280.0));
        obstacle.translation.y = thread_rng().gen_range(-(CENTER_Y - 20.0)..(CENTER_Y - 20.0));
    }

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
    if player1.translation.y < -CENTER_Y || player1.translation.y > CENTER_Y {
        game_state.health_amount = 0;
    }

    // Move road objects
    for sprite in engine.sprites.values_mut() {
        if sprite.label.starts_with("roadline") {
            sprite.translation.x -= ROAD_SPEED * engine.delta_f32;
            if sprite.translation.x < -CENTER_X {
                sprite.translation.x += ROAD_SPACING * 10.0;
            }
        }
        if sprite.label.starts_with("obstacle") {
            sprite.translation.x -= ROAD_SPEED * engine.delta_f32;
            if sprite.translation.x < -CENTER_X {
                sprite.translation.x = thread_rng().gen_range(CENTER_X..(WINDOW_WIDTH + 280.0));
                sprite.translation.y = thread_rng().gen_range(-(CENTER_Y - 20.0)..(CENTER_Y - 20.0));
            }
        }    }
}