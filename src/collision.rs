use std::time::Duration;
use bevy::prelude::*;
use bevy::time::common_conditions::on_timer;
use kd_tree::{KdPoint, KdTree};

use crate::consts::*;
use crate::gun::Bullet;
use crate::characters::monsters::Monster;
use crate::player::{Player, PlayerEnemyCollisionEvent};
use crate::state::GameState;

pub struct CollisionPlugin;

#[derive(Component)]
struct Collidable {
    pos: Vec2,
    entity: Entity,
}

impl KdPoint for Collidable {
    type Scalar = f32;
    type Dim = typenum::U2;
    fn at(&self, k: usize) -> f32 {
        if k == 0 {
            return self.pos.x;
        }

        self.pos.y
    }
}

#[derive(Resource)]
struct EnemyKdTree(KdTree<Collidable>);

impl Default for EnemyKdTree {
    fn default() -> Self {
        Self(KdTree::build_by_ordered_float(vec![]))
    }
}

impl Plugin for CollisionPlugin {
    fn build(&self, app: &mut App) {
        app
            .insert_resource(EnemyKdTree::default())
            .add_systems(
                Update,
                (
                    handle_enemy_player_collision,
                    handle_enemy_bullet_collision,
                    update_enemy_kd_tree.run_if(on_timer(Duration::from_secs_f32(KD_TREE_REFRESH_RATE))),
                )
                    .run_if(in_state(GameState::InGame)),
            );
    }
}

fn update_enemy_kd_tree(
    mut tree: ResMut<EnemyKdTree>,
    enemy_query: Query<(&Transform, Entity), With<Monster>>,
) {
    let mut items = Vec::new();
    for (transform, entity) in enemy_query.iter() {
        items.push(Collidable {
            entity,
            pos: transform.translation.truncate(),
        })
    }

    tree.0 = KdTree::build_by_ordered_float(items);
}

fn handle_enemy_bullet_collision(
    mut commands: Commands,
    bullet_query: Query<(&Transform, Entity), With<Bullet>>,
    tree: Res<EnemyKdTree>,
    mut enemy_query: Query<(&Transform, &mut Monster), With<Monster>>,
) {
    if bullet_query.is_empty() || enemy_query.is_empty() {
        return;
    }

    for (bullet_transform, entity) in bullet_query.iter() {
        let pos = bullet_transform.translation;
        let enemies = tree.0.within_radius(&[pos.x, pos.y], 25.0);

        for e in enemies {
            if let Ok((_, mut enemy)) = enemy_query.get_mut(e.entity) {
                if let Some(mut entity_command) = commands.get_entity(entity) {
                    // enemy.health -= BULLET_DAMAGE;
                    entity_command.despawn();
                }
            }
        }
    }
}

fn handle_enemy_player_collision(
    mut commands: Commands,
    player_query: Query<&Transform, With<Player>>,
    tree: Res<EnemyKdTree>,
    mut ew: EventWriter<PlayerEnemyCollisionEvent>,
) {
    if player_query.is_empty() {
        return;
    }

    let player_pos = player_query.single().translation;
    let enemies = tree.0.within_radius(&[player_pos.x, player_pos.y], 50.0);
    for c in enemies.iter() {
        if let Some(mut entity_command) = commands.get_entity(c.entity) {
            ew.send(PlayerEnemyCollisionEvent);
            entity_command.despawn();
        }
    }
}
