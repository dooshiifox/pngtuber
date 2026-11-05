pub mod config;

use bevy::{platform::collections::HashMap, prelude::*};

use crate::{
    image::config::{Face, FaceEntry},
    volume::Volume,
};

pub struct FacePlugin;
impl Plugin for FacePlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, insert_faces)
            .add_systems(Update, (keyboard, update_image).chain());
    }
}

#[derive(Resource, Reflect)]
#[reflect(Resource)]
pub struct Faces {
    pub faces: HashMap<String, FaceEntry<Handle<Image>>>,
    pub current: String,
}
impl Faces {
    pub fn active(&self) -> &FaceEntry<Handle<Image>> {
        self.faces
            .get(&self.current)
            .unwrap_or_else(|| self.faces.get("default").unwrap())
    }

    pub fn set_current(&mut self, current: impl Into<String>) {
        self.current = current.into();
    }
}

#[derive(Component)]
pub struct FaceSprite;

#[derive(bevy::ecs::system::SystemParam)]
pub struct CurrentFace<'w> {
    faces: Res<'w, Faces>,
    volume: Volume<'w>,
}
impl<'w> CurrentFace<'w> {
    pub fn volume(&self) -> f32 {
        self.volume.volume()
    }
    pub fn face_entry(&self) -> &FaceEntry<Handle<Image>> {
        self.faces.active()
    }

    pub fn face(&self) -> &Face<Handle<Image>> {
        let active = self.face_entry();
        let volume = self.volume();
        if volume > 0. {
            active.talk_sprite.as_ref().unwrap_or(&active.idle_sprite)
        } else {
            &active.idle_sprite
        }
    }
}

fn insert_faces(mut commands: Commands) {
    commands.spawn((Sprite::default(), FaceSprite));
}

fn keyboard(keyboard: Res<ButtonInput<KeyCode>>, mut faces: ResMut<Faces>) {
    // info!("{:?}", keyboard.get_pressed().collect::<Vec<&KeyCode>>());
    let mut new = None;
    for (name, about) in &faces.faces {
        if about.activate.is_pressed(&keyboard) {
            new = Some(name.clone());
        }
    }

    if let Some(new) = new {
        faces.set_current(new);
    }
}

fn update_image(
    current_face: CurrentFace,
    mut images: Query<(&mut Transform, &mut Sprite), With<FaceSprite>>,
    time: Res<Time>,
) {
    let active = current_face.face_entry();
    let volume = current_face.volume();
    let face = current_face.face();
    for (mut transform, mut sprite) in images.iter_mut() {
        sprite.image = face.image.clone();
        sprite.rect = face.crop;
        sprite.custom_size = face.size;
        sprite.image_mode = SpriteImageMode::Scale(face.scaling_mode.into());
        sprite.flip_x = face.flip_x;
        sprite.flip_y = face.flip_y;

        transform.translation.x = face.position.x;
        transform.translation.y.smooth_nudge(
            &(face.position.y + volume * active.talk_bounce_amount),
            20.,
            time.delta_secs(),
        );
    }
}
