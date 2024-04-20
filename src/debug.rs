use bevy::prelude::*;
use belly::prelude::*;
use bevy::diagnostic::{DiagnosticsStore, FrameTimeDiagnosticsPlugin, LogDiagnosticsPlugin};
use belly::widgets::common::Label;

use crate::enemy::Enemy;
use crate::state::GameState;

pub struct DebugPlugin;

#[derive(Component, Default)]
struct DebugMenu {
    fps: f32,
    num_enemies: usize,
}

#[derive(Debug, Clone, Eq, PartialEq, Hash, Copy, Default, States)]
enum DebugMenuState {
    #[default]
    Hidden,
    Visible,
}

impl Plugin for DebugPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_plugins((
                FrameTimeDiagnosticsPlugin,
                LogDiagnosticsPlugin::default(),
            ))
            .init_state::<DebugMenuState>()
            .add_systems(OnEnter(GameState::Loading), load_assets)
            .add_systems(OnEnter(GameState::GameInit), spawn_debugmenu)
            .add_systems(
                Update,
                (
                    process_input,
                    (
                        fetch_debug_data,
                        update_debug_menu_text,
                    ).run_if(in_state(DebugMenuState::Visible)),
                ).run_if(in_state(GameState::InGame)),
            );
    }
}

fn load_assets(
    mut commands: Commands,
) {
    commands.add(StyleSheet::load("debugmenu.css"));
}

fn spawn_debugmenu(
    mut commands: Commands,
) {
    let debug_menu = commands.spawn_empty().id();
    commands.add(eml! {
        <div id="debugmenu" c:debugmenu c:hidden>
            <label {debug_menu} with=DebugMenu />
        </div>
    });
}

fn fetch_debug_data(
    mut query: Query<&mut DebugMenu>,
    diagnostics: Res<DiagnosticsStore>,
    enemy_query: Query<(), With<Enemy>>,
) {
    if query.is_empty() || enemy_query.is_empty() {
        return;
    }

    let mut debug_menu = query.single_mut();

    debug_menu.num_enemies = enemy_query.iter().count();
    if let Some(fps) = diagnostics.get(&FrameTimeDiagnosticsPlugin::FPS) {
        if let Some(value) = fps.smoothed() {
            debug_menu.fps = value as f32;
        }
    }
}

fn update_debug_menu_text(
    mut query: Query<(&DebugMenu, &mut Label), Changed<DebugMenu>>,
) {
    if query.is_empty() {
        return;
    }

    let (debug_menu, mut label) = query.single_mut();

    label.value = format!(
        "FPS: {:.2}\nEnemies: {}",
        debug_menu.fps,
        debug_menu.num_enemies,
    );
}

fn process_input(
    keyboard_input: Res<ButtonInput<KeyCode>>,
    current_state: Res<State<DebugMenuState>>,
    mut next_state: ResMut<NextState<DebugMenuState>>,
    mut elements: Elements,
) {
    if keyboard_input.just_pressed(KeyCode::F3) {
        match current_state.get() {
            DebugMenuState::Visible => {
                next_state.set(DebugMenuState::Hidden);
                elements.select("#debugmenu").add_class("hidden");
            },
            DebugMenuState::Hidden => {
                next_state.set(DebugMenuState::Visible);
                elements.select("#debugmenu").remove_class("hidden");
            },
        }
    }
}
