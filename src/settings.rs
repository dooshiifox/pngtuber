use std::{fs::File, io::Write};

use bevy::{
    asset::io::embedded::GetAssetServer,
    platform::{collections::HashMap, dirs::preferences_dir},
    prelude::*,
    settings::SaveSettingsSync,
    window::WindowCloseRequested,
};
use serde::{Deserialize, Serialize};

use crate::image::{
    Faces,
    config::{Face, FaceEntry},
};

pub struct SettingsPlugin {
    pub app_name: &'static str,
}
impl Plugin for SettingsPlugin {
    fn build(&self, app: &mut App) {
        let faces = load_face_settings(app, self.app_name).unwrap_or_else(|| {
            let asset_loader = app.get_asset_server();
            Faces {
                current: "default".to_string(),
                faces: HashMap::from_iter([(
                    "default".into(),
                    FaceEntry {
                        activate: "SM-h".into(),
                        talk_bounce_amount: 50.,
                        idle_sprite: Face {
                            image: asset_loader.load("test.png"),
                            ..default()
                        },
                        talk_sprite: None,
                    },
                )]),
            }
        });
        app.add_plugins(bevy::settings::SettingsPlugin::new(self.app_name))
            .insert_resource(faces)
            .insert_resource(AppName(self.app_name))
            .add_systems(Update, on_window_close);
    }
}

#[derive(
    Resource, Clone, Eq, PartialEq, PartialOrd, Ord, Deref, DerefMut, Reflect, Debug, Hash,
)]
struct AppName(&'static str);

#[derive(Reflect, Serialize, Deserialize)]
struct FaceSettings(HashMap<String, FaceEntry<String>>);

fn ron() -> ron::Options {
    use ron::extensions::Extensions;
    ron::Options::default().with_default_extension(
        Extensions::UNWRAP_NEWTYPES
            | Extensions::IMPLICIT_SOME
            | Extensions::UNWRAP_VARIANT_NEWTYPES,
    )
}

fn faces_save_dir(app_name: &str) -> Option<std::path::PathBuf> {
    let Some(base_dir) = preferences_dir() else {
        error!("No settings directory found");
        return None;
    };
    Some(base_dir.join(app_name).join("faces.ron"))
}

fn load_face_settings(app: &mut App, app_name: &str) -> Option<Faces> {
    let file_path = faces_save_dir(app_name)?;
    if !file_path.exists() {
        warn!("Face settings don't exist");
        return None;
    }

    let settings_str = match std::fs::read_to_string(file_path) {
        Ok(settings_str) => settings_str,
        Err(e) => {
            panic!("Error reading settings file: {e}");
        }
    };
    let settings: FaceSettings = match ron().from_str(&settings_str) {
        Ok(settings) => settings,
        Err(err) => {
            panic!("Error reading settings file: {err}");
        }
    };

    let asset_loader = app.get_asset_server();
    let mut faces = Faces {
        faces: HashMap::new(),
        current: "default".to_string(),
    };
    for (name, entry) in settings.0 {
        let entry = entry.to_app(asset_loader);
        faces.faces.insert(name, entry);
    }

    if !faces.faces.contains_key("default") {
        panic!("A face with the key `default` is required but not found")
    }
    Some(faces)
}

fn save_face_settings(faces: &Faces, app_name: &str) {
    let Some(file_path) = faces_save_dir(app_name) else {
        warn!("Not saving settings");
        return;
    };

    let mut settings = FaceSettings(HashMap::new());
    for (name, entry) in &faces.faces {
        let entry = entry.to_settings();
        settings.0.insert(name.clone(), entry);
    }
    let settings_str =
        match ron().to_string_pretty(&settings, ron::ser::PrettyConfig::new().indentor("\t")) {
            Ok(str) => str,
            Err(err) => {
                error!("could not save settings: {err}");
                return;
            }
        };

    let mut file = match File::create(file_path) {
        Ok(file) => file,
        Err(err) => {
            error!("Could not create settings file: {err}");
            return;
        }
    };
    match file.write_all(settings_str.as_bytes()) {
        Ok(file) => file,
        Err(err) => {
            error!("Could not write to settings file: {err}");
            return;
        }
    };

    trace!("saved settings")
}

/// A Command which saves face settings to disk.
/// This blocks the command queue until saving is complete.
#[derive(Default, PartialEq)]
pub struct SaveFaceSettingsSync;

impl Command for SaveFaceSettingsSync {
    type Out = ();

    fn apply(self, world: &mut World) {
        save_face_settings(world.resource(), world.resource::<AppName>());
    }
}

fn on_window_close(mut close: MessageReader<WindowCloseRequested>, mut commands: Commands) {
    // Save settings immediately, then quit.
    if close.read().next().is_some() {
        commands.queue(SaveSettingsSync::Always);
        commands.queue(SaveFaceSettingsSync);
        commands.write_message(AppExit::Success);
    }
}
