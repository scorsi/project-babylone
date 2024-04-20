use std::f32::consts::PI;
use std::time::Instant;

use bevy::math::{vec2, vec3};
use bevy::prelude::*;
use bevy::time::Stopwatch;
use rand::Rng;

use crate::consts::*;
use crate::player::Player;
use crate::state::GameState;
use crate::resources::{CursorPosition, GlobalTextureAtlas};
use crate::world::GameEntity;

pub struct GunPlugin;

#[derive(Component)]
pub struct Gun;

#[derive(Component)]
pub struct GunTimer(pub Stopwatch);

#[derive(Component)]
pub struct Bullet;

#[derive(Component)]
struct BulletDirection(Vec2);

#[derive(Component)]
pub struct SpawnInstant(Instant);

impl Plugin for GunPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            (
                update_gun_transform,
                update_bullets,
                despawn_old_bullets,
                handle_gun_input,
                flip_gun_sprite_y,
            )
                .run_if(in_state(GameState::InGame)),
        );
    }
}

fn update_gun_transform(
    cursor_pos: Res<CursorPosition>,
    player_query: Query<&Transform, With<Player>>,
    mut gun_query: Query<&mut Transform, (With<Gun>, Without<Player>)>,
) {
    if player_query.is_empty() || gun_query.is_empty() {
        return;
    }

    let player_pos = player_query.single().translation.truncate();
    let cursor_pos = match cursor_pos.0 {
        Some(pos) => pos,
        None => player_pos,
    };
    let mut gun_transform = gun_query.single_mut();

    let angle = (player_pos.y - cursor_pos.y).atan2(player_pos.x - cursor_pos.x) + PI;
    gun_transform.rotation = Quat::from_rotation_z(angle);

    let offset = 20.0;
    let new_gun_pos = vec2(
        player_pos.x + offset * angle.cos() - 5.0,
        player_pos.y + offset * angle.sin() - 10.0,
    );

    gun_transform.translation = vec3(new_gun_pos.x, new_gun_pos.y, gun_transform.translation.z);
    gun_transform.translation.z = 15.0;
}

fn handle_gun_input(
    mut commands: Commands,
    time: Res<Time>,
    mut gun_query: Query<(&Transform, &mut GunTimer), With<Gun>>,
    mouse_button_input: Res<ButtonInput<MouseButton>>,
    handle: Res<GlobalTextureAtlas>,
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
    let mut rng = rand::thread_rng();

    let gun_direction = gun_transform.rotation.mul_vec3(Vec3::X).truncate();

    for _ in 0..NUM_BULLETS_PER_SHOT {
        let bullet_spread_angle = rng.gen_range(-BULLET_MAX_SPREADING_ANGLE..BULLET_MAX_SPREADING_ANGLE);
        let bullet_direction = vec2(
            gun_direction.x * bullet_spread_angle.cos() - gun_direction.y * bullet_spread_angle.sin(),
            gun_direction.x * bullet_spread_angle.sin() + gun_direction.y * bullet_spread_angle.cos(),
        );

        commands.spawn((
            SpriteSheetBundle {
                texture: handle.image.clone().unwrap(),
                atlas: TextureAtlas {
                    layout: handle.layout.clone().unwrap(),
                    index: 16,
                },
                transform: Transform::from_translation(Vec3::new(gun_pos.x, gun_pos.y, BULLET_Z_INDEX))
                    .with_scale(Vec3::splat(SPRITE_SCALE_FACTOR)),
                ..default()
            },
            Bullet,
            BulletDirection(bullet_direction.normalize()),
            GameEntity,
            SpawnInstant(Instant::now()),
        ));
    }
}

fn update_bullets(
    time: Res<Time>,
    mut bullet_query: Query<(&mut Transform, &BulletDirection), With<Bullet>>,
) {
    if bullet_query.is_empty() {
        return;
    }

    for (mut t, dir) in bullet_query.iter_mut() {
        t.translation += dir.0.normalize().extend(0.0) * Vec3::splat(BULLET_SPEED) * time.delta_seconds();
        t.translation.z = 10.0;
    }
}

fn despawn_old_bullets(
    mut commands: Commands,
    bullet_query: Query<(&SpawnInstant, Entity), With<Bullet>>,
) {
    for (instant, e) in bullet_query.iter() {
        if instant.0.elapsed().as_secs_f32() > BULLET_LIFETIME {
            commands.entity(e).despawn();
        }
    }
}

fn flip_gun_sprite_y(
    cursor_position: Res<CursorPosition>,
    mut gun_query: Query<(&mut Sprite, &Transform), With<Gun>>,
) {
    if gun_query.is_empty() {
        return;
    }

    let (mut sprite, transform) = gun_query.single_mut();
    if let Some(cursor_position) = cursor_position.0 {
        if cursor_position.x > transform.translation.x {
            sprite.flip_y = false;
        } else {
            sprite.flip_y = true;
        }
    }
}
