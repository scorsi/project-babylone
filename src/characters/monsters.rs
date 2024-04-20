use std::f32::consts::PI;
use std::time::Duration;
use bevy::prelude::*;
use bevy::asset::{Asset, AssetServer, Handle};
use bevy::math::vec2;
use bevy::time::common_conditions::on_timer;
use bevy::utils::HashMap;
use bevy_aseprite::Aseprite;
use bevy_aseprite::anim::AsepriteAnimation;
use leafwing_manifest::identifier::Id;
use leafwing_manifest::manifest::{Manifest, ManifestFormat};
use leafwing_manifest::plugin::RegisterManifest;
use serde::{Deserialize, Serialize};
use rand::prelude::*;

use crate::common::health::Health;
use crate::player::Player;
use crate::state::GameState;
use crate::world::GameEntity;

pub const MAX_NUM_MONSTERS: usize = 10000;
pub const MONSTER_SPAWN_INTERVAL: f32 = 1.0;
pub const MONSTER_SPAWN_RATE_PER_SECOND: usize = 2;
pub const MONSTER_Z_INDEX: f32 = 9.0;

pub(crate) struct MonstersPlugin;

impl Plugin for MonstersPlugin {
    fn build(&self, app: &mut App) {
        app
            .register_manifest::<MonsterManifest>("monsters/data.ron")
            .add_systems(
                PostUpdate,
                (
                    spawn_monsters.run_if(on_timer(Duration::from_secs_f32(1.0))),
                    move_monsters_towards_player,
                    flip_monster_sprite,
                ).run_if(in_state(GameState::InGame)),
            );
    }
}

#[derive(Debug, PartialEq, Component, Clone)]
pub(crate) struct Monster {
    pub id: Id<MonsterData>,
    pub speed: f32,
}

#[derive(Debug, Bundle)]
pub(crate) struct MonsterBundle {
    pub monster: Monster,
    pub aseprite: Handle<Aseprite>,
    pub animation: AsepriteAnimation,
    pub transform: Transform,
    pub health: Health,
}

fn flip_monster_sprite(
    player_query: Query<&Transform, With<Player>>,
    mut monster_query: Query<(&mut Sprite, &Transform), With<Monster>>,
) {
    if player_query.is_empty() || monster_query.is_empty() {
        return;
    }

    let player_pos = player_query.single().translation;
    for (mut sprite, transform) in monster_query.iter_mut() {
        if transform.translation.x < player_pos.x {
            sprite.flip_x = false;
        } else {
            sprite.flip_x = true;
        }
    }
}

fn move_monsters_towards_player(
    time: Res<Time>,
    player_query: Query<&Transform, (With<Player>, Without<Monster>)>,
    mut monster_query: Query<(&mut Transform, &Monster), With<Monster>>,
) {
    if player_query.is_empty() {
        return;
    }

    let player_pos = player_query.single().translation.truncate();

    for (mut monster_transform, monster) in monster_query.iter_mut() {
        let enemy_pos = monster_transform.translation.truncate();
        let direction = player_pos - enemy_pos;
        let distance = direction.length();
        let direction = direction / distance;

        monster_transform.translation += direction.extend(0.0) * monster.speed * 100.0 * time.delta_seconds();
    }
}

fn spawn_monsters(
    mut commands: Commands,
    monster_manifest: Res<MonsterManifest>,
    player_query: Query<&Transform, (With<Player>, Without<Monster>)>,
    monster_query: Query<&Transform, With<Monster>>,
) {
    if player_query.is_empty() {
        return;
    }

    let num_monsters = monster_query.iter().count();
    if num_monsters >= MAX_NUM_MONSTERS {
        return;
    }
    let spawn_count = (MAX_NUM_MONSTERS - num_monsters).min(MONSTER_SPAWN_RATE_PER_SECOND);
    let player_pos = player_query.single().translation.truncate();

    let mut rng = thread_rng();

    let monster_ids = monster_manifest.0.keys().collect::<Vec<_>>();

    for _ in 0..spawn_count {
        let monster_id = *monster_ids.choose(&mut rng).unwrap();
        let monster_data = monster_manifest.0.get(monster_id).unwrap();
        let monster_pos = get_random_position_around(player_pos).extend(MONSTER_Z_INDEX);

        commands
            .spawn(MonsterBundle {
                transform: Transform::from_translation(monster_pos).with_scale(Vec3::splat(3.0)),
                aseprite: monster_data.sprite.clone(),
                animation: AsepriteAnimation::from("walk"),
                monster: Monster {
                    id: *monster_id,
                    speed: monster_data.speed,
                },
                health: Health(monster_data.health),
            })
            .insert(GameEntity);
    }
}

fn get_random_position_around(pos: Vec2) -> Vec2 {
    let mut rng = thread_rng();
    let angle = rng.gen_range(0.0..PI * 2.0);
    let dist = rng.gen_range(1000.0..5000.0);

    let offset_x = angle.cos() * dist;
    let offset_y = angle.sin() * dist;

    let random_x = pos.x + offset_x;
    let random_y = pos.y + offset_y;

    vec2(random_x, random_y)
}

#[derive(Debug, PartialEq, Component)]
pub(crate) struct MonsterData {
    pub name: String,
    pub health: f32,
    // pub attack: f32,
    // pub defense: f32,
    pub speed: f32,
    pub sprite: Handle<Aseprite>,
}

#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub struct RawMonsterData {
    name: String,
    health: f32,
    // attack: f32,
    // defense: f32,
    speed: f32,
    sprite: String,
}

#[derive(Debug, Resource, PartialEq)]
pub(crate) struct MonsterManifest(pub HashMap<Id<MonsterData>, MonsterData>);

#[derive(Debug, Asset, TypePath, Serialize, Deserialize, PartialEq)]
pub struct RawMonsterManifest(Vec<RawMonsterData>);

impl Manifest for MonsterManifest {
    type RawManifest = RawMonsterManifest;
    type RawItem = RawMonsterData;
    type Item = MonsterData;
    type ConversionError = std::convert::Infallible;

    const FORMAT: ManifestFormat = ManifestFormat::Ron;

    fn from_raw_manifest(
        raw_manifest: Self::RawManifest,
        world: &mut World,
    ) -> Result<Self, Self::ConversionError> {
        // Asset server to load our sprite assets
        let asset_server = world.resource::<AssetServer>();

        let monsters: HashMap<_, _> = raw_manifest
            .0
            .into_iter()
            .map(|raw_item| {
                let sprite_handle = asset_server.load(raw_item.sprite);

                let item = MonsterData {
                    name: raw_item.name,
                    health: raw_item.health,
                    // attack: raw_item.attack,
                    // defense: raw_item.defense,
                    speed: raw_item.speed,
                    sprite: sprite_handle,
                };

                let id = Id::from_name(&item.name);

                (id, item)
            })
            .collect();

        Ok(MonsterManifest(monsters))
    }

    fn get(&self, id: Id<MonsterData>) -> Option<&Self::Item> {
        self.0.get(&id)
    }
}
