use bevy::prelude::*;

use crate::{debug::DebugHideable, image::CurrentFace};

pub struct ImageDebugPlugin;
impl Plugin for ImageDebugPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, (add_image_background, update_image_background));
    }
}

#[derive(Clone, Copy, Component, Default, Eq, PartialEq, Hash, Reflect, Debug)]
struct FaceDebugBackground;

fn add_image_background(
    images: Query<Entity, With<crate::image::FaceSprite>>,
    backgrounds: Query<&ChildOf, With<FaceDebugBackground>>,
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    let images_without_child = images
        .iter()
        .filter(|entity| backgrounds.related(*entity).is_none());
    for childless in images_without_child {
        commands.entity(childless).with_child((
            FaceDebugBackground,
            DebugHideable,
            Mesh2d(meshes.add(Rectangle::new(1., 1.))),
            MeshMaterial2d(materials.add(Color::BLACK)),
            Transform::default(),
        ));
    }
}

fn update_image_background(
    current_face: CurrentFace,
    assets: Res<Assets<Image>>,
    mut backgrounds: Query<&mut Transform, With<FaceDebugBackground>>,
) {
    let face = current_face.face();
    for mut bg in &mut backgrounds {
        bg.scale = face
            .size
            .unwrap_or_else(|| assets.get(&face.image).unwrap().size_f32())
            .extend(1.);
    }
}
