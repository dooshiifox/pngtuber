mod image;
mod volume;

use bevy::prelude::*;

use crate::debug::{image::ImageDebugPlugin, volume::VolumeDebugPlugin};

pub struct DebugPlugin;
impl Plugin for DebugPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins((VolumeDebugPlugin, ImageDebugPlugin))
            .add_systems(Update, (DebugFlag::system, DebugFlag::hide).chain())
            .insert_resource(DebugFlag(cfg!(debug_assertions)));
    }
}

#[derive(Clone, Copy, Resource, Default, Eq, PartialEq, Hash, Reflect, Debug, Deref, DerefMut)]
pub struct DebugFlag(pub bool);
impl DebugFlag {
    fn system(keyboard: Res<ButtonInput<KeyCode>>, mut flag: ResMut<DebugFlag>) {
        if keyboard.just_pressed(KeyCode::KeyD) {
            flag.0 = !flag.0;
        }
    }

    fn hide(flag: Res<DebugFlag>, mut hideable: Query<&mut Visibility, With<DebugHideable>>) {
        for mut visibility in hideable.iter_mut() {
            *visibility = if **flag {
                Visibility::Visible
            } else {
                Visibility::Hidden
            };
        }
    }
}

#[derive(Clone, Copy, Component, Default, Eq, PartialEq, Hash, Reflect, Debug)]
struct DebugHideable;
