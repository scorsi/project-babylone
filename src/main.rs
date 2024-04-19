use std::f32::consts::PI;
use bevy::diagnostic::{FrameTimeDiagnosticsPlugin, LogDiagnosticsPlugin};
use bevy::input::mouse::MouseButtonInput;
use bevy::math::vec2;
use bevy::prelude::*;
use bevy::time::Stopwatch;
use bevy::window::{close_on_esc, PrimaryWindow};

const WW: f32 = 1200.0;
const WH: f32 = 900.0;
const BG_COLOR: (u8, u8, u8) = (25, 20, 43);

const SPRITE_SHEET_PATH: &str = "assets.png";
const SPRITE_SCALE_FACTOR: f32 = 3.0;
const SPRITE_SHEET_W: usize = 8;
const SPRITE_SHEET_H: usize = 8;
const TILE_W: usize = 16;
const TILE_H: usize = 16;

const BULLET_SPAWN_INTERVAL: f32 = 0.2;

#[derive(Resource)]
struct GlobalTextureAtlasHandle(Option<Handle<TextureAtlasLayout>>);

#[derive(Resource)]
struct GlobalSpriteSheetHandle(Option<Handle<Image>>);

#[derive(Resource)]
struct CursorPos(Option<Vec2>);

#[derive(Debug, Clone, Eq, PartialEq, Hash, Copy, Default, States)]
enum GameState {
    #[default]
    Loading,
    GameInit,
    InGame,
}

const PLAYER_SPEED: f32 = 2.0;

#[derive(Component)]
struct Player;

#[derive(Component)]
struct Gun;

#[derive(Component)]
struct GunTimer(Stopwatch);

#[derive(Component)]
struct Bullet;

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
        .insert_resource(CursorPos(None))
        .add_systems(OnEnter(GameState::Loading), load_assets)
        .add_systems(OnEnter(GameState::GameInit), (setup_camera, init_world))
        .add_systems(Update, (update_cursor_pos, handle_player_input, handle_gun_input, update_gun_transform).run_if(in_state(GameState::InGame)))
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

fn init_world(
    mut commands: Commands,
    texture_atlas_handle: Res<GlobalTextureAtlasHandle>,
    sprite_sheet_handle: Res<GlobalSpriteSheetHandle>,
    mut game_state: ResMut<NextState<GameState>>,
) {
    commands.spawn((
        SpriteSheetBundle {
            texture: sprite_sheet_handle.0.clone().unwrap(),
            atlas: TextureAtlas {
                layout: texture_atlas_handle.0.clone().unwrap(),
                index: 0,
            },
            transform: Transform::from_scale(Vec3::splat(SPRITE_SCALE_FACTOR)),
            ..default()
        },
        Player,
    ));
    commands.spawn((
        SpriteSheetBundle {
            texture: sprite_sheet_handle.0.clone().unwrap(),
            atlas: TextureAtlas {
                layout: texture_atlas_handle.0.clone().unwrap(),
                index: 17,
            },
            transform: Transform::from_scale(Vec3::splat(SPRITE_SCALE_FACTOR)),
            ..default()
        },
        Gun,
        GunTimer(Stopwatch::new()),
    ));

    game_state.set(GameState::InGame);
}

fn handle_player_input(
    mut player_query: Query<&mut Transform, With<Player>>,
    keyboard_input: Res<ButtonInput<KeyCode>>,
) {
    if player_query.is_empty() {
        return;
    }

    let mut transform = player_query.single_mut();

    let w_key = keyboard_input.pressed(KeyCode::KeyW) || keyboard_input.pressed(KeyCode::ArrowUp);
    let s_key = keyboard_input.pressed(KeyCode::KeyS) || keyboard_input.pressed(KeyCode::ArrowDown);
    let a_key = keyboard_input.pressed(KeyCode::KeyA) || keyboard_input.pressed(KeyCode::ArrowLeft);
    let d_key = keyboard_input.pressed(KeyCode::KeyD) || keyboard_input.pressed(KeyCode::ArrowRight);

    let mut delta = Vec2::ZERO;
    if w_key {
        delta.y += 1.0;
    }
    if s_key {
        delta.y -= 1.0;
    }
    if a_key {
        delta.x -= 1.0;
    }
    if d_key {
        delta.x += 1.0;
    }
    delta = delta.normalize();

    if delta.is_finite() && delta != Vec2::ZERO {
        transform.translation += Vec3::new(delta.x, delta.y, 0.0) * PLAYER_SPEED;
    }
}

fn handle_gun_input(
    mut commands: Commands,
    time: Res<Time>,
    mut gun_query: Query<(&Transform, &mut GunTimer), With<Gun>>,
    texture_atlas_handle: Res<GlobalTextureAtlasHandle>,
    sprite_sheet_handle: Res<GlobalSpriteSheetHandle>,
    mouse_button_input: Res<ButtonInput<MouseButton>>,
) {
    if gun_query.is_empty() {
        return;
    }

    let (gun_transform, mut gun_timer) = gun_query.single_mut();
    let gun_pos = gun_transform.translation.truncate();
    gun_timer.0.tick(time.delta());

    if !mouse_button_input.pressed(MouseButton::Left) || gun_timer.0.elapsed_secs() < BULLET_SPAWN_INTERVAL {
        return;
    }

    gun_timer.0.reset();

    commands.spawn((
        SpriteSheetBundle {
            texture: sprite_sheet_handle.0.clone().unwrap(),
            atlas: TextureAtlas {
                layout: texture_atlas_handle.0.clone().unwrap(),
                index: 16,
            },
            transform: Transform::from_translation(Vec3::new(gun_pos.x, gun_pos.y, 1.0))
                .with_scale(Vec3::splat(SPRITE_SCALE_FACTOR)),
            ..default()
        },
        Bullet,
    ));
}

fn update_cursor_pos(
    mut cursor_pos: ResMut<CursorPos>,
    window_query: Query<&Window, With<PrimaryWindow>>,
    camera_query: Query<(&Camera, &GlobalTransform), With<Camera>>,
) {
    if window_query.is_empty() || camera_query.is_empty() {
        cursor_pos.0 = None;
        return;
    }

    let (camera, camera_transform) = camera_query.single();
    let window = window_query.single();

    cursor_pos.0 = window
        .cursor_position()
        .and_then(|cursor_pos| camera.viewport_to_world(camera_transform, cursor_pos))
        .map(|ray| ray.origin.truncate());
}

fn update_gun_transform(
    cursor_pos: Res<CursorPos>,
    player_query: Query<&Transform, With<Player>>,
    mut gun_query: Query<&mut Transform, (With<Gun>, Without<Player>)>,
) {
    if player_query.is_empty() || gun_query.is_empty() {
        return;
    }

    let player_pos = player_query.single().translation.truncate();
    let cursor_pos = cursor_pos.0.unwrap_or(player_pos);
    let mut gun_transform = gun_query.single_mut();

    let angle = (player_pos.y - cursor_pos.y).atan2(player_pos.x - cursor_pos.x) + PI;
    gun_transform.rotation = Quat::from_rotation_z(angle);

    let offset = 20.0;
    let new_gun_pos = vec2(
        player_pos.x + offset * angle.cos() - 5.0,
        player_pos.y + offset * angle.sin() - 15.0,
    );

    gun_transform.translation = Vec3::new(new_gun_pos.x, new_gun_pos.y, gun_transform.translation.z);
}
