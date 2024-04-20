use bevy::prelude::*;

use crate::consts::*;
use crate::resources::CursorPosition;
use crate::state::GameState;

pub struct PlayerPlugin;

#[derive(Component)]
pub struct Player;

#[derive(Component, Default)]
pub enum PlayerState {
    #[default]
    Idle,
    Run,
}

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_systems(
                Update,
                (
                    handle_player_input,
                    flip_player_sprite_x,
                )
                    .run_if(in_state(GameState::InGame)),
            );
    }
}

fn handle_player_input(
    time: Res<Time>,
    mut player_query: Query<(&mut Transform, &mut PlayerState), With<Player>>,
    keyboard_input: Res<ButtonInput<KeyCode>>,
) {
    if player_query.is_empty() {
        return;
    }

    let (mut transform, mut player_state) = player_query.single_mut();
    let w_key = keyboard_input.pressed(KeyCode::KeyW) || keyboard_input.pressed(KeyCode::ArrowUp);
    let a_key = keyboard_input.pressed(KeyCode::KeyA) || keyboard_input.pressed(KeyCode::ArrowLeft);
    let s_key = keyboard_input.pressed(KeyCode::KeyS) || keyboard_input.pressed(KeyCode::ArrowDown);
    let d_key =
        keyboard_input.pressed(KeyCode::KeyD) || keyboard_input.pressed(KeyCode::ArrowRight);

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
        transform.translation += Vec3::new(delta.x, delta.y, 0.0) * PLAYER_SPEED * time.delta_seconds();
        *player_state = PlayerState::Run;
    } else {
        *player_state = PlayerState::Idle;
    }
}

fn flip_player_sprite_x(
    cursor_position: Res<CursorPosition>,
    mut player_query: Query<(&mut Sprite, &Transform), With<Player>>,
) {
    if player_query.is_empty() {
        return;
    }

    let (mut sprite, transform) = player_query.single_mut();
    if let Some(cursor_position) = cursor_position.0 {
        if cursor_position.x > transform.translation.x {
            sprite.flip_x = false;
        } else {
            sprite.flip_x = true;
        }
    }
}
