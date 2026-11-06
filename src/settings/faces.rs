use std::{fs::File, io::Write, sync::atomic::AtomicBool};

use bevy::{
    platform::{collections::HashMap, dirs::preferences_dir},
    prelude::*,
};
use serde::{Deserialize, Serialize};

use crate::{
    image::{
        Faces,
        config::{Face, FaceEntry},
    },
    settings::watch::watch_file,
};

pub struct FacesSettingsPlugin {
    pub app_name: &'static str,
}
impl FacesSettingsPlugin {
    fn init_faces_config(&self, app: &mut App) {
        let faces =
            load_face_settings(app.world().resource(), self.app_name).unwrap_or_else(|| {
                let asset_loader: &AssetServer = app.world().resource();
                Faces {
                    current: "default".to_string(),
                    faces: HashMap::from_iter([(
                        "default".into(),
                        FaceEntry {
                            activate: "SM-h".into(),
                            talk_bounce_amount: crate::image::config::default_talk_bounce_amount(),
                            stop_talking_delay: crate::image::config::default_stop_talking_delay(),
                            idle_sprite: Face {
                                image: asset_loader.load("test.png"),
                                ..default()
                            },
                            talk_sprite: None,
                        },
                    )]),
                }
            });
        app.insert_resource(faces);
    }

    fn setup_file_watcher(&self, app: &mut App) {
        let Some(config_file) = get_faces_save_path(self.app_name) else {
            return;
        };

        let Some(debouncer) =
            watch_file(config_file.parent().unwrap().to_path_buf(), move |event| {
                info!("Got event! {event:?}");
                HAS_FACES_CONFIG_CHANGED.store(true, std::sync::atomic::Ordering::Relaxed);
            })
        else {
            return;
        };

        #[derive(Resource)]
        struct FileWatcher(
            #[allow(
                dead_code,
                reason = "This is required so that `debouncer` isn't dropped while the program is running."
            )]
            notify_debouncer_full::Debouncer<
                notify_debouncer_full::notify::RecommendedWatcher,
                notify_debouncer_full::RecommendedCache,
            >,
        );
        app.add_systems(Update, reload_face_settings)
            .insert_resource(FileWatcher(debouncer));
    }
}
impl Plugin for FacesSettingsPlugin {
    fn build(&self, app: &mut App) {
        self.init_faces_config(app);
        self.setup_file_watcher(app);
    }
}

static HAS_FACES_CONFIG_CHANGED: AtomicBool = AtomicBool::new(false);

/// A Command which saves face settings to disk.
/// This blocks the command queue until saving is complete.
#[derive(Default, PartialEq)]
pub struct SaveFaceSettingsSync;

impl Command for SaveFaceSettingsSync {
    type Out = ();

    fn apply(self, world: &mut World) {
        save_face_settings(world.resource(), world.resource::<super::AppName>());
    }
}

fn ron() -> ron::Options {
    use ron::extensions::Extensions;
    ron::Options::default().with_default_extension(
        Extensions::UNWRAP_NEWTYPES
            | Extensions::IMPLICIT_SOME
            | Extensions::UNWRAP_VARIANT_NEWTYPES,
    )
}

#[derive(Reflect, Serialize, Deserialize)]
struct FaceSettings(HashMap<String, FaceEntry<String>>);

fn get_faces_save_path(app_name: &str) -> Option<std::path::PathBuf> {
    let Some(base_dir) = preferences_dir() else {
        error!("No settings directory found");
        return None;
    };
    Some(base_dir.join(app_name).join("faces.ron"))
}

fn load_face_settings(asset_loader: &AssetServer, app_name: &str) -> Option<Faces> {
    let file_path = get_faces_save_path(app_name)?;
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

fn reload_face_settings(
    asset_server: Res<AssetServer>,
    app_name: Res<super::AppName>,
    mut commands: Commands,
) {
    if HAS_FACES_CONFIG_CHANGED.swap(false, std::sync::atomic::Ordering::Relaxed)
        && let Some(config) = load_face_settings(&asset_server, &app_name)
    {
        commands.insert_resource_if_neq(config);
    }
}

fn save_face_settings(faces: &Faces, app_name: &str) {
    let Some(file_path) = get_faces_save_path(app_name) else {
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
