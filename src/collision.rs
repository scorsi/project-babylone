use bevy::prelude::*;
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
    enemy_query: Query<(&Transform, Entity), (With<Enemy>, Without<Bullet>)>,
) {
    for (bullet_transform, bullet_entity) in bullet_query.iter() {
        for (enemy_transform, enemy_entity) in enemy_query.iter() {
            if bullet_transform.translation.distance_squared(enemy_transform.translation) <= 100.0 {
                commands.entity(bullet_entity).despawn();
                commands.entity(enemy_entity).despawn();
            }
        }
    }
}
