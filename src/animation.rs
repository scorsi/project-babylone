use bevy::prelude::*;
use crate::enemy::Enemy;

use crate::player::Player;
use crate::state::GameState;

pub struct AnimationPlugin;

#[derive(Component, Deref, DerefMut)]
pub struct AnimationTimer(pub Timer);

impl Plugin for AnimationPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_systems(
                Update,
                (
                    animation_timer_tick,
                    animate_player,
                    animate_enemy,
                ).run_if(in_state(GameState::InGame)),
            );
    }
}

fn animation_timer_tick(
    time: Res<Time>,
    mut query: Query<&mut AnimationTimer, With<AnimationTimer>>,
) {
    for mut timer in query.iter_mut() {
        timer.tick(time.delta());
    }
}

fn animate_player(
    time: Res<Time>,
    mut player_query: Query<(&mut TextureAtlas, &AnimationTimer), With<Player>>,
) {
    if player_query.is_empty() {
        return;
    }

    let (mut texture, timer) = player_query.single_mut();
    if timer.just_finished() {
        texture.index = (texture.index + 1) % 4;
    }
}

fn animate_enemy(
    mut enemy_query: Query<(&mut TextureAtlas, &AnimationTimer), With<Enemy>>,
) {
    if enemy_query.is_empty() {
        return;
    }

    for (mut atlas, timer) in enemy_query.iter_mut() {
        if timer.just_finished() {
            atlas.index = 8 + (atlas.index + 1) % 4;
        }
    }
}