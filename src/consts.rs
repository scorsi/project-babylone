pub const WW: f32 = 1200.0;
pub const WH: f32 = 900.0;
pub const BG_COLOR: (u8, u8, u8) = (25, 20, 43);

pub const SPRITE_SHEET_PATH: &str = "assets.png";
pub const SPRITE_SCALE_FACTOR: f32 = 3.0;
pub const SPRITE_SHEET_W: usize = 8;
pub const SPRITE_SHEET_H: usize = 8;
pub const TILE_W: usize = 16;
pub const TILE_H: usize = 16;

pub const PLAYER_Z_INDEX: f32 = 10.0;
pub const GUN_Z_INDEX: f32 = 11.0;
pub const BULLET_Z_INDEX: f32 = 1.0;
pub const WORLD_DECORATION_Z_INDEX: f32 = 0.0;
pub const ENEMY_Z_INDEX: f32 = 9.0;

pub const BULLET_SPAWN_INTERVAL: f32 = 0.25;
pub const BULLET_SPEED: f32 = 700.0;
pub const BULLET_DAMAGE: f32 = 15.0;
pub const BULLET_LIFETIME: f32 = 0.5;
pub const NUM_BULLETS_PER_SHOT: usize = 5;
pub const BULLET_MAX_SPREADING_ANGLE: f32 = 0.15;

pub const WORLD_W: f32 = 3000.0;
pub const WORLD_H: f32 = 3000.0;
pub const NUM_WORLD_DECORATIONS: usize = 500;


pub const PLAYER_SPEED: f32 = 300.0;
pub const PLAYER_MAX_HEALTH: f32 = 10.0;

pub const MAX_NUM_ENEMIES: usize = 10000;
pub const ENEMY_SPAWN_INTERVAL: f32 = 1.0;
pub const ENEMY_SPAWN_RATE_PER_SECOND: usize = 2;
pub const ENEMY_SPEED: f32 = 225.0;
pub const ENEMY_MAX_HEALTH: f32 = 100.0;
pub const ENEMY_DAMAGE: f32 = 1.0;

pub const KD_TREE_REFRESH_RATE: f32 = 0.1;
