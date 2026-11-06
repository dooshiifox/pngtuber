use bevy::prelude::*;

use crate::{debug::DebugHideable, volume::Volume};

pub struct VolumeDebugPlugin;
impl Plugin for VolumeDebugPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, spawn_volume_bar)
            .add_systems(Update, (update_volume_bar, update_volume_text));
    }
}

#[derive(Clone, Copy, Component, Default, Eq, PartialEq, Hash, Reflect, Debug)]
struct VolumeBar;
#[derive(Clone, Copy, Component, Default, Eq, PartialEq, Hash, Reflect, Debug)]
struct VolumeText;

fn spawn_volume_bar(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    window: Single<&Window>,
) {
    commands
        .spawn((
            Name::new("VolumeBarBackground"),
            DebugHideable,
            Mesh2d(meshes.add(Rectangle::new(200., 10.))),
            MeshMaterial2d({
                materials.add(ColorMaterial::from_color(Color::srgb(0.2, 0.2, 0.2)))
            }),
            Transform::from_xyz(-window.width() / 2. + 110., window.height() / 2. - 20., 10.),
        ))
        .with_children(|spawner| {
            spawner.spawn((
                Name::new("VolumeBarColor"),
                VolumeBar,
                Mesh2d(meshes.add(Rectangle::new(200., 10.))),
                MeshMaterial2d({
                    materials.add(ColorMaterial::from_color(Color::srgb(1.0, 0.2, 0.2)))
                }),
                Transform::from_xyz(0., 0., 10.),
            ));
        });

    commands.spawn((
        Name::new("VolumeBarText"),
        VolumeText,
        DebugHideable,
        Text::new("..."),
        Node {
            top: px(30),
            left: px(10),
            ..default()
        },
    ));
}

fn update_volume_bar(speech: Volume, mut volume: Query<(&mut Transform,), With<VolumeBar>>) {
    for (mut transform,) in volume.iter_mut() {
        let vol = speech.unclamped_dbfs();
        transform.scale = Vec3::new(vol.remap(-60., 0., 0.01, 1.), 1., 1.)
    }
}

fn update_volume_text(speech: Volume, mut query: Query<&mut Text, With<VolumeText>>) {
    for mut span in query.iter_mut() {
        let vol = speech.unclamped_dbfs();
        **span = format!("{:.1} dB", vol);
    }
}
