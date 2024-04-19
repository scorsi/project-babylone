use bevy::diagnostic::{FrameTimeDiagnosticsPlugin, LogDiagnosticsPlugin};
use bevy::prelude::*;
use bevy::window::close_on_esc;

const WW: f32 = 1200.0;
const WH: f32 = 900.0;
const BG_COLOR: (u8, u8, u8) = (25, 20, 43);

const SPRITE_SHEET_PATH: &str = "assets.png";
const SPRITE_SCALE_FACTOR: f32 = 3.0;
const SPRITE_SHEET_W: usize = 8;
const SPRITE_SHEET_H: usize = 8;
const TILE_W: usize = 16;
const TILE_H: usize = 16;

#[derive(Resource)]
struct GlobalTextureAtlasHandle(Option<Handle<TextureAtlasLayout>>);

#[derive(Resource)]
struct GlobalSpriteSheetHandle(Option<Handle<Image>>);

#[derive(Debug, Clone, Eq, PartialEq, Hash, Copy, Default, States)]
enum GameState {
    #[default]
    Loading,
    GameInit,
    InGame,
}

fn main() {
    App::new()
        .add_plugins(
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
        )
        .insert_resource(Msaa::Off)

        .add_plugins(LogDiagnosticsPlugin::default())
        .add_plugins(FrameTimeDiagnosticsPlugin)

        .insert_resource(ClearColor(Color::rgb_u8(
            BG_COLOR.0, BG_COLOR.1, BG_COLOR.2,
        )))

        .init_state::<GameState>()
        .insert_resource(GlobalTextureAtlasHandle(None))
        .insert_resource(GlobalSpriteSheetHandle(None))
        .add_systems(OnEnter(GameState::Loading), load_assets)
        .add_systems(OnEnter(GameState::GameInit), (setup_camera, spawn_player))
        .add_systems(Update, close_on_esc)

        .run();
}

fn load_assets(
    mut texture_atlas_handle: ResMut<GlobalTextureAtlasHandle>,
    mut sprite_sheet_handle: ResMut<GlobalSpriteSheetHandle>,
    asset_server: Res<AssetServer>,
    mut texture_atlas_layouts: ResMut<Assets<TextureAtlasLayout>>,
    mut game_state: ResMut<NextState<GameState>>,
) {
    sprite_sheet_handle.0 = Some(asset_server.load(SPRITE_SHEET_PATH));
    let layout = TextureAtlasLayout::from_grid(
        Vec2::new(TILE_W as f32, TILE_H as f32),
        SPRITE_SHEET_W, SPRITE_SHEET_H,
        None, None,
    );
    texture_atlas_handle.0 = Some(texture_atlas_layouts.add(layout));

    game_state.set(GameState::GameInit);
}

fn setup_camera(mut commands: Commands) {
    commands.spawn(Camera2dBundle::default());
}

fn spawn_player(
    mut commands: Commands,
    texture_atlas_handle: Res<GlobalTextureAtlasHandle>,
    sprite_sheet_handle: Res<GlobalSpriteSheetHandle>,
) {
    commands.spawn(
        SpriteSheetBundle {
            texture: sprite_sheet_handle.0.clone().unwrap(),
            atlas: TextureAtlas {
                layout: texture_atlas_handle.0.clone().unwrap(),
                index: 0,
            },
            transform: Transform::from_scale(Vec3::splat(SPRITE_SCALE_FACTOR)),
            ..default()
        },
    );
}
