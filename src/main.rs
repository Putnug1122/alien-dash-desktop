#![allow(unused)]

use bevy::prelude::*;
use player::PlayerPlugin;

mod components;
mod player;
// region:       -- Asset Constant

const PLAYER_SPRITE: &str = "player_a_01.png";
const PLAYER_SIZE: (f32, f32) = (144., 75.);

const SPRITE_SCALE: f32 = 0.5;

// endregion:   -- Asset Constant

// region:      -- Resources

pub struct WinSize {
    pub w: f32,
    pub h: f32,
}

struct GameTexture {
    player: Handle<Image>,
}

// endregion:   -- Resources

// region
const TIME_STEP: f32 = 1. / 60.;
const BASE_SPEED: f32 = 500.;

// endregion
fn main() {
    App::new()
        .insert_resource(ClearColor(Color::rgb(0.04, 0.04, 0.04)))
        .insert_resource(WindowDescriptor {
            title: "Rust Invaders".to_string(),
            width: 598.0,
            height: 676.0,
            ..Default::default()
        })
        .add_plugins(DefaultPlugins)
        .add_plugin(PlayerPlugin)
        .add_startup_system(setup_system)
        .run()
}

fn setup_system(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut windows: ResMut<Windows>,
) {
    commands.spawn_bundle(OrthographicCameraBundle::new_2d());

    let window = windows.get_primary_mut().unwrap();
    let (win_w, win_h) = (window.width(), window.height());

    window.set_position(IVec2::new(2780, 4900));

    let win_size = WinSize { w: win_h, h: win_h };
    commands.insert_resource(win_size);

    let game_texture = GameTexture {
        player: asset_server.load(PLAYER_SPRITE),
    };
    commands.insert_resource(game_texture);
}
