use bevy::prelude::*;
use crate::consts::BULLET_DAMAGE;
use crate::enemy::Enemy;
use crate::gun::Bullet;

use crate::state::GameState;

pub struct CollisionPlugin;

impl Plugin for CollisionPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_systems(
                Update,
                (
                    // handle_player_enemy_collision,
                    handle_enemy_bullet_collision,
                )
                    .run_if(in_state(GameState::InGame)),
            );
    }
}

fn handle_enemy_bullet_collision(
    mut commands: Commands,
    bullet_query: Query<(&Transform, Entity), With<Bullet>>,
    mut enemy_query: Query<(&Transform, &mut Enemy), (With<Enemy>, Without<Bullet>)>,
) {
    for (bullet_transform, bullet_entity) in bullet_query.iter() {
        for (enemy_transform, mut enemy) in enemy_query.iter_mut() {
            if bullet_transform.translation.distance_squared(enemy_transform.translation) <= 100.0 {
                enemy.health -= BULLET_DAMAGE;
                commands.entity(bullet_entity).despawn();
            }
        }
    }
}
