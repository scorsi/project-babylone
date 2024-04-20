use bevy::prelude::*;
use leafwing_manifest::asset_state::{AssetLoadingState, SimpleAssetState};
use leafwing_manifest::plugin::ManifestPlugin;
use crate::state::GameState;

pub(super) struct AssetsPlugin;

// #[derive(PartialEq, Eq, Debug, Hash, Clone, Copy, Default, States)]
// pub enum AssetsState {
//     #[default]
//     Loading,
//     Processing,
//     Validating,
//     Ready,
//     Failed,
// }
//
// impl AssetLoadingState for AssetsState {
//     const LOADING: Self = AssetsState::Loading;
//     const PROCESSING: Self = AssetsState::Processing;
//     const READY: Self = AssetsState::Ready;
//     const FAILED: Self = AssetsState::Failed;
// }

impl Plugin for AssetsPlugin {
    fn build(&self, app: &mut App) {
        app
            .init_state::<SimpleAssetState>()
            .add_plugins(ManifestPlugin::<SimpleAssetState>::default())
            .add_systems(OnEnter(SimpleAssetState::Ready), assets_loaded);
    }
}

fn assets_loaded(
    mut next_state: ResMut<NextState<GameState>>,
) {
    next_state.set(GameState::MainMenu);
}
