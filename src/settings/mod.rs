pub mod faces;
pub mod watch;

use bevy::{prelude::*, settings::SaveSettingsSync, window::WindowCloseRequested};

use crate::settings::faces::SaveFaceSettingsSync;

pub struct SettingsPlugin {
    pub app_name: &'static str,
}
impl Plugin for SettingsPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins((
            bevy::settings::SettingsPlugin::new(self.app_name),
            faces::FacesSettingsPlugin {
                app_name: self.app_name,
            },
        ))
        .insert_resource(AppName(self.app_name))
        .add_systems(Update, on_window_close);
    }
}

#[derive(
    Resource, Clone, Eq, PartialEq, PartialOrd, Ord, Deref, DerefMut, Reflect, Debug, Hash,
)]
struct AppName(&'static str);

fn on_window_close(mut close: MessageReader<WindowCloseRequested>, mut commands: Commands) {
    // Save settings immediately, then quit.
    if close.read().next().is_some() {
        commands.queue(SaveSettingsSync::Always);
        commands.queue(SaveFaceSettingsSync);
        commands.write_message(AppExit::Success);
    }
}
