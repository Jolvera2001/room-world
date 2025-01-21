use bevy::prelude::*;
use bevy_common_assets::ron::RonAssetPlugin;
use serde::Deserialize;

use super::interact::InteractType;
pub struct DialogPlugin;

impl Plugin for DialogPlugin {
    fn build(&self, app: &mut App) {
        app
        .add_plugins(RonAssetPlugin::<DialogAsset>::new(&["dialog_test.ron"]))
        .add_systems(Update, fetch_dialog_data);
    }
}

#[derive(Asset, TypePath, Debug, Deserialize)]
pub struct DialogAsset {
    lines: Vec<String>,
}

// marker
#[derive(Component)]
pub struct DialogEntity;

#[derive(Component)]
pub struct DialogData {
    dialog_file: Handle<DialogAsset>
}

fn fetch_dialog_data(
    dialog_event: EventReader<InteractType>,
    dialog_entity: Query<(&DialogData, Entity), With<DialogEntity>>,
) {
    for event in dialog_event.read() {
        if event.contains()
    }
}