pub mod config;

use bevy::{platform::collections::HashMap, prelude::*};

use crate::{
    image::config::{Face, FaceEntry},
    volume::Volume,
};

pub struct FacePlugin;
impl Plugin for FacePlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(StopTalkingDelayTimer::default())
            .add_systems(Startup, insert_faces)
            .add_systems(
                Update,
                (
                    update_active_image,
                    StopTalkingDelayTimer::tick,
                    update_image,
                )
                    .chain(),
            );
    }
}

#[derive(Resource, Reflect, PartialEq, Clone, Debug)]
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

#[derive(Resource, Default)]
pub struct StopTalkingDelayTimer(Timer);
impl StopTalkingDelayTimer {
    fn tick(
        mut timer: ResMut<StopTalkingDelayTimer>,
        time: Res<Time>,
        volume: Volume,
        faces: Res<Faces>,
    ) {
        timer.0.set_duration(faces.active().stop_talking_delay);
        if volume.volume() > 0. {
            timer.0.reset();
        } else {
            timer.0.tick(time.delta());
        }
    }
}

#[derive(bevy::ecs::system::SystemParam)]
pub struct CurrentFace<'w> {
    faces: Res<'w, Faces>,
    volume: Volume<'w>,
    stop_talking_delay_timer: Res<'w, StopTalkingDelayTimer>,
}
impl<'w> CurrentFace<'w> {
    pub fn volume(&self) -> f32 {
        self.volume.volume()
    }
    pub fn face_entry(&self) -> &FaceEntry<Handle<Image>> {
        self.faces.active()
    }
    pub fn is_talking(&self) -> bool {
        let volume = self.volume();
        volume > 0. || !self.stop_talking_delay_timer.0.is_finished()
    }

    pub fn face(&self) -> &Face<Handle<Image>> {
        let active = self.face_entry();
        if self.is_talking() {
            active.talk_sprite.as_ref().unwrap_or(&active.idle_sprite)
        } else {
            &active.idle_sprite
        }
    }
}

fn insert_faces(mut commands: Commands) {
    commands.spawn((Sprite::default(), FaceSprite));
}

fn update_active_image(keyboard: Res<ButtonInput<KeyCode>>, mut faces: ResMut<Faces>) {
    // info!("{:?}", keyboard.get_pressed().collect::<Vec<&KeyCode>>());
    let mut new = None;
    for (name, about) in &faces.faces {
        if about.activate.is_pressed(&keyboard) {
            new = Some(name);
        }
    }

    if let Some(new) = new.cloned() {
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
