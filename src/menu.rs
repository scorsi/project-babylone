use bevy::prelude::*;
use belly::prelude::*;
use crate::state::GameState;

pub struct MenuPlugin;

impl Plugin for MenuPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_systems(OnEnter(GameState::Loading), load_assets)
            .add_systems(OnEnter(GameState::MainMenu), spawn_menu)
            .add_systems(OnExit(GameState::MainMenu), despawn_menu);
    }
}

fn load_assets(
    mut commands: Commands,
) {
    commands.add(StyleSheet::load("mainmenu.css"));
}

fn spawn_menu(mut commands: Commands) {
    commands.add(eml! {
        <div c:menu>
            <label value="Project Babylone" c:title/>
            <button
                on:press=run!(|ctx| {
                    ctx.add(|world: &mut World| {
                        world.resource_mut::<NextState<GameState>>().set(GameState::GameInit);
                    });
                })
            >
                <label value="Play"/>
            </button>
        </div>
    });
}

fn despawn_menu(
    mut elements: Elements,
    mut focused: ResMut<belly::core::input::Focused>,
) {
    elements.select(".menu").remove();
    *focused = belly::core::input::Focused::default();
}
