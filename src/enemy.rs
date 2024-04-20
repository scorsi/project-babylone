use std::f32::consts::PI;
use std::time::Duration;
use bevy::math::{vec2, vec3};
use bevy::prelude::*;
use bevy::time::common_conditions::on_timer;
use rand::Rng;
use crate::animation::AnimationTimer;

use crate::consts::*;
use crate::player::Player;
use crate::resources::GlobalTextureAtlas;
use crate::state::GameState;

pub struct EnemyPlugin;

#[derive(Component)]
pub struct Enemy {
    pub health: f32,
}

impl Default for Enemy {
    fn default() -> Self {
        Self { health: 100.0 }
    }
}

impl Plugin for EnemyPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_systems(
                Update,
                (
                    spawn_enemies.run_if(on_timer(Duration::from_secs_f32(ENEMY_SPAWN_INTERVAL))),
                    update_enemy_transform,
                    despawn_dead_enemies,
                    flip_enemy_sprite_x,
                ).run_if(in_state(GameState::InGame)),
            );
    }
}

fn spawn_enemies(
    mut commands: Commands,
    handle: Res<GlobalTextureAtlas>,
    player_query: Query<&Transform, With<Player>>,
    enemy_query: Query<&Transform, (With<Enemy>, Without<Player>)>,
) {
    if player_query.is_empty() {
        return;
    }

    let num_enemies = enemy_query.iter().count();
    let enemy_spawn_count = (MAX_NUM_ENEMIES - num_enemies).min(ENEMY_SPAWN_RATE_PER_SECOND);

    if num_enemies >= MAX_NUM_ENEMIES {
        return;
    }

    let player_pos = player_query.single().translation.truncate();

    for _ in 0..enemy_spawn_count {
        let enemy_pos = get_random_position_around(player_pos).extend(ENEMY_Z_INDEX);

        commands.spawn((
            SpriteSheetBundle {
                texture: handle.image.clone().unwrap(),
                atlas: TextureAtlas {
                    layout: handle.layout.clone().unwrap(),
                    index: 8,
                },
                transform: Transform::from_translation(enemy_pos).with_scale(Vec3::splat(SPRITE_SCALE_FACTOR)),
                ..default()
            },
            Enemy::default(),
            AnimationTimer(Timer::from_seconds(0.1, TimerMode::Repeating)),
        ));
    }
}

fn get_random_position_around(pos: Vec2) -> Vec2 {
    let mut rng = rand::thread_rng();
    let angle = rng.gen_range(0.0..PI * 2.0);
    let dist = rng.gen_range(1000.0..5000.0);

    let offset_x = angle.cos() * dist;
    let offset_y = angle.sin() * dist;

    let random_x = pos.x + offset_x;
    let random_y = pos.y + offset_y;

    vec2(random_x, random_y)
}

fn update_enemy_transform(
    time: Res<Time>,
    mut enemy_query: Query<&mut Transform, With<Enemy>>,
    player_query: Query<&Transform, (With<Player>, Without<Enemy>)>,
) {
    if player_query.is_empty() {
        return;
    }

    let player_pos = player_query.single().translation.truncate();

    for mut enemy_transform in enemy_query.iter_mut() {
        let enemy_pos = enemy_transform.translation.truncate();
        let direction = player_pos - enemy_pos;
        let distance = direction.length();
        let direction = direction / distance;

        enemy_transform.translation += direction.extend(0.0) * ENEMY_SPEED * time.delta_seconds();
    }
}

fn flip_enemy_sprite_x(
    player_query: Query<&Transform, With<Player>>,
    mut enemy_query: Query<(&mut Sprite, &Transform), With<Enemy>>,
) {
    if player_query.is_empty() || enemy_query.is_empty() {
        return;
    }

    let player_pos = player_query.single().translation;
    for (mut sprite, transform) in enemy_query.iter_mut() {
        if transform.translation.x < player_pos.x {
            sprite.flip_x = false;
        } else {
            sprite.flip_x = true;
        }
    }
}

fn despawn_dead_enemies(mut commands: Commands, enemy_query: Query<(&Enemy, Entity), With<Enemy>>) {
    if enemy_query.is_empty() {
        return;
    }

    for (enemy, entity) in enemy_query.iter() {
        if enemy.health <= 0.0 {
            commands.entity(entity).despawn();
        }
    }
}
