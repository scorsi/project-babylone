pub(crate) mod consts;
pub(crate) mod resources;
pub(crate) mod state;
pub(crate) mod world;
pub(crate) mod player;
pub(crate) mod gun;
pub(crate) mod camera;
pub(crate) mod animation;
pub(crate) mod collision;
pub(crate) mod mainmenu;
pub(crate) mod debug;
mod assets;
pub(crate) mod common;
pub(crate) mod characters;

use bevy::prelude::*;
use bevy::window::close_on_esc;
use belly::prelude::*;
use bevy::app::AppExit;
use bevy::utils::HashMap;
use bevy::window::PresentMode::AutoNoVsync;
use bevy_aseprite::AsepritePlugin;
use clap::Parser;
use leafwing_manifest::{
    asset_state::SimpleAssetState,
    identifier::Id,
    manifest::{Manifest, ManifestFormat},
    plugin::{ManifestPlugin, RegisterManifest},
};
use serde::{Deserialize, Serialize};

use crate::consts::*;
use crate::gun::GunPlugin;
use crate::player::PlayerPlugin;
use crate::resources::ResourcesPlugin;
use crate::state::GameState;
use crate::world::WorldPlugin;
use crate::camera::CameraPlugin;
use crate::animation::AnimationPlugin;
use crate::assets::AssetsPlugin;
use crate::collision::CollisionPlugin;
use crate::debug::DebugPlugin;
use crate::mainmenu::MainMenuPlugin;
use crate::characters::monsters::MonstersPlugin;

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
                        present_mode: AutoNoVsync,
                        ..default()
                    }),
                    ..default()
                }),
            BellyPlugin,
            AsepritePlugin,
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
            AnimationPlugin,
            CollisionPlugin,
            MainMenuPlugin,
            DebugPlugin,
            // new modules
            AssetsPlugin,
            MonstersPlugin,
        ))
        .add_systems(Update, close_on_esc)

        .run();
}
