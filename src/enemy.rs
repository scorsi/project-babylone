use std::time::Duration;
use bevy::math::vec3;
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
pub struct Enemy;

impl Plugin for EnemyPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_systems(
                Update,
                (
                    spawn_enemies.run_if(on_timer(Duration::from_secs_f32(ENEMY_SPAWN_INTERVAL))),
                    update_enemy_transform,
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
    let enemy_spawn_count = (MAX_NUM_ENEMIES - num_enemies).min(10);

    if num_enemies >= MAX_NUM_ENEMIES {
        return;
    }

    let player_pos = player_query.single().translation.truncate();
    let mut rng = rand::thread_rng();

    for _ in 0..enemy_spawn_count {
        let enemy_pos = vec3(
            rng.gen_range(-WORLD_W..WORLD_W),
            rng.gen_range(-WORLD_H..WORLD_H),
            ENEMY_Z_INDEX,
        );

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
            Enemy,
            AnimationTimer(Timer::from_seconds(0.1, TimerMode::Repeating)),
        ));
    }
}

fn update_enemy_transform(
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

        enemy_transform.translation += direction.extend(0.0) * ENEMY_SPEED;
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
