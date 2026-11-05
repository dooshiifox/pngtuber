use bevy::prelude::*;
use serde::{Deserialize, Serialize};

use crate::input::KeyComboList;

#[derive(Reflect, Clone, PartialEq, Debug, Serialize, Deserialize)]
pub struct FaceEntry<ImageType: Default> {
    pub activate: KeyComboList,
    #[serde(default = "default_talk_bounce_amount")]
    pub talk_bounce_amount: f32,
    pub idle_sprite: Face<ImageType>,
    #[serde(default)]
    pub talk_sprite: Option<Face<ImageType>>,
}
const fn default_talk_bounce_amount() -> f32 {
    50.
}
impl FaceEntry<Handle<Image>> {
    pub fn to_settings(&self) -> FaceEntry<String> {
        FaceEntry {
            activate: self.activate.clone(),
            talk_bounce_amount: self.talk_bounce_amount,
            idle_sprite: self.idle_sprite.to_settings(),
            talk_sprite: self.talk_sprite.as_ref().map(Face::to_settings),
        }
    }
}
impl FaceEntry<String> {
    pub fn to_app(&self, asset_server: &AssetServer) -> FaceEntry<Handle<Image>> {
        FaceEntry {
            activate: self.activate.clone(),
            talk_bounce_amount: self.talk_bounce_amount,
            idle_sprite: self.idle_sprite.to_app(asset_server),
            talk_sprite: self
                .talk_sprite
                .as_ref()
                .map(|sprite| sprite.to_app(asset_server)),
        }
    }
}

#[derive(Reflect, Clone, PartialEq, Debug, Serialize, Deserialize, Default)]
pub struct Face<ImageType> {
    pub image: ImageType,
    /// The region in the source image to use.
    /// If not provided, defaults to the full image.
    #[serde(default)]
    pub crop: Option<Rect>,
    /// Where to put the image on the output.
    #[serde(default)]
    pub position: Vec2,
    /// How big the output should be. Can be used to scale the output.
    /// Defaults to the full image.
    ///
    /// If this isn't the same aspect ratio as [`region`], the scaling method
    /// in [`scaling_mode`] is used -- by default this centers the image such
    /// that it doesn't clip.
    #[serde(default)]
    pub size: Option<Vec2>,
    #[serde(default)]
    pub scaling_mode: SpriteScalingMode,
    /// Flip the image horizontally. Applied after [`crop`]
    #[serde(default)]
    pub flip_x: bool,
    /// Flip the image vertically. Applied after [`crop`]
    #[serde(default)]
    pub flip_y: bool,
}
impl Face<Handle<Image>> {
    pub fn to_settings(&self) -> Face<String> {
        Face::<String> {
            image: self.image.path().expect("not a valid path?").to_string(),
            crop: self.crop,
            position: self.position,
            size: self.size,
            scaling_mode: self.scaling_mode,
            flip_x: self.flip_x,
            flip_y: self.flip_y,
        }
    }
}
impl Face<String> {
    pub fn to_app(&self, asset_server: &AssetServer) -> Face<Handle<Image>> {
        Face::<Handle<Image>> {
            image: asset_server
                .load_builder()
                .override_unapproved()
                .load(&self.image),
            crop: self.crop,
            position: self.position,
            size: self.size,
            scaling_mode: self.scaling_mode,
            flip_x: self.flip_x,
            flip_y: self.flip_y,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Default, Reflect, Serialize, Deserialize)]
#[reflect(Debug, Default, Clone)]
pub enum SpriteScalingMode {
    FillCenter,
    FillStart,
    FillEnd,
    #[default]
    FitCenter,
    FitStart,
    FitEnd,
}
impl From<SpriteScalingMode> for bevy::prelude::SpriteScalingMode {
    fn from(value: SpriteScalingMode) -> Self {
        match value {
            SpriteScalingMode::FillCenter => bevy::prelude::SpriteScalingMode::FillCenter,
            SpriteScalingMode::FillStart => bevy::prelude::SpriteScalingMode::FillStart,
            SpriteScalingMode::FillEnd => bevy::prelude::SpriteScalingMode::FillEnd,
            SpriteScalingMode::FitCenter => bevy::prelude::SpriteScalingMode::FitCenter,
            SpriteScalingMode::FitStart => bevy::prelude::SpriteScalingMode::FitStart,
            SpriteScalingMode::FitEnd => bevy::prelude::SpriteScalingMode::FitEnd,
        }
    }
}
