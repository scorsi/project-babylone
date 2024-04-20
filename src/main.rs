pub(crate) mod consts;
pub(crate) mod resources;
pub(crate) mod state;
pub(crate) mod world;
pub(crate) mod player;
pub(crate) mod gun;
pub(crate) mod camera;
pub(crate) mod enemy;
pub(crate) mod animation;
pub(crate) mod collision;
pub(crate) mod mainmenu;
pub(crate) mod debug;

use bevy::prelude::*;
use bevy::window::close_on_esc;
use belly::prelude::*;

use crate::consts::*;
use crate::gun::GunPlugin;
use crate::player::PlayerPlugin;
use crate::resources::ResourcesPlugin;
use crate::state::GameState;
use crate::world::WorldPlugin;
use crate::camera::CameraPlugin;
use crate::enemy::EnemyPlugin;
use crate::animation::AnimationPlugin;
use crate::collision::CollisionPlugin;
use crate::debug::DebugPlugin;
use crate::mainmenu::MainMenuPlugin;

fn main() {
    App::new()
        .add_plugins((
            DefaultPlugins
                .set(ImagePlugin::default_nearest())
                .set(WindowPlugin {
                    primary_window: Some(Window {
                        resizable: true,
                        focused: true,
                        resolution: (WW, WH).into(),
                        ..default()
                    }),
                    ..default()
                }),
            BellyPlugin,
        ))
        .insert_resource(Msaa::Off)

        .insert_resource(ClearColor(Color::rgb_u8(
            BG_COLOR.0, BG_COLOR.1, BG_COLOR.2,
        )))

        .init_state::<GameState>()
        .add_plugins((
            ResourcesPlugin,
            WorldPlugin,
            PlayerPlugin,
            GunPlugin,
            CameraPlugin,
            EnemyPlugin,
            AnimationPlugin,
            CollisionPlugin,
            MainMenuPlugin,
            DebugPlugin,
        ))
        .add_systems(Update, close_on_esc)

        .run();
}
