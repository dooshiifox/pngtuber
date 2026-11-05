pub mod debug;
mod image;
pub mod input;
pub mod settings;
mod volume;

#[allow(unused_imports)]
#[cfg(debug_assertions)]
#[allow(clippy::single_component_path_imports)]
use bevy_dylib;

use bevy::prelude::*;

use crate::{image::FacePlugin, settings::SettingsPlugin, volume::VolumeInput};

fn main() {
    App::new()
        .add_plugins((
            DefaultPlugins
                .set(WindowPlugin {
                    // We want to intercept the exit so that we can save settings.
                    exit_condition: bevy::window::ExitCondition::DontExit,

                    primary_window: Some(Window {
                        title: "PNGTUBER".into(),
                        name: Some("PNGTUBER".into()),
                        position: WindowPosition::Centered(MonitorSelection::Current),
                        composite_alpha_mode: bevy::window::CompositeAlphaMode::PreMultiplied,
                        transparent: true,
                        ..default()
                    }),
                    ..default()
                })
                .set(AssetPlugin {
                    unapproved_path_mode: bevy::asset::UnapprovedPathMode::Deny,
                    ..default()
                }),
            debug::DebugPlugin,
            FacePlugin,
            SettingsPlugin {
                app_name: "dev.dooshii.pngtuber",
            },
        ))
        .insert_resource(ClearColor(Color::srgba(0., 0., 0., 0.)))
        .insert_resource(VolumeInput::new())
        .add_systems(Startup, setup_camera)
        .run();
}

fn setup_camera(mut commands: Commands) {
    commands.spawn((Camera2d,));
}
