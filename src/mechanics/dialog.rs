use bevy::prelude::*;
use bevy_common_assets::ron::RonAssetPlugin;
use serde::Deserialize;

use super::interact::InteractType;
pub struct DialogPlugin;

impl Plugin for DialogPlugin {
    fn build(&self, app: &mut App) {
        app.init_asset::<DialogAsset>()
            .add_plugins(RonAssetPlugin::<DialogAsset>::new(&["dialog_test.ron"]))
            .add_event::<DialogEvent>()
            .add_systems(Update, fetch_dialog_data);
    }
}

#[derive(Asset, TypePath, Debug, Deserialize, Clone)]
pub struct DialogAsset {
    pub lines: Vec<String>,
}

#[derive(Event)]
pub enum DialogEvent {
    DialogData(DialogAsset),
}

// marker
#[derive(Component)]
pub struct DialogEntity;

#[derive(Component)]
pub struct DialogData {
    pub dialog_file: Handle<DialogAsset>,
}

fn fetch_dialog_data(
    mut dialog_event: EventReader<InteractType>,
    mut event_writer: EventWriter<DialogEvent>,
    dialog_query: Query<(&DialogData, Entity), With<DialogEntity>>,
    dialog_assets: Res<Assets<DialogAsset>>,
) {
    for event in dialog_event.read() {
        if let InteractType::Dialog(entity) = event {
            if let Ok((dialog_data, _)) = dialog_query.get(*entity) {
                if let Some(asset) = dialog_assets.get(&dialog_data.dialog_file) {
                    event_writer.send(DialogEvent::DialogData(asset.clone()));
                }
            }
        }
    }
}
