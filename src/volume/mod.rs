use std::{
    sync::{
        Arc,
        atomic::{AtomicU16, Ordering},
    },
    time::Duration,
};

use bevy::{
    math::ops::log10,
    prelude::*,
    settings::{ReflectSettingsGroup, SettingsGroup},
};
use cpal::{
    BufferSize, StreamConfig,
    traits::{DeviceTrait, HostTrait, StreamTrait},
};

#[derive(bevy::ecs::system::SystemParam)]
pub struct Volume<'w> {
    input: Res<'w, VolumeInput>,
    settings: Res<'w, VolumeSettings>,
}
impl<'w> Volume<'w> {
    pub fn unclamped_dbfs(&self) -> f32 {
        let raw_vol = self.input.raw_volume();
        20. * log10(raw_vol)
    }

    pub fn dbfs(&self) -> Option<f32> {
        let dbfs = self.unclamped_dbfs();

        if dbfs < self.settings.threshold {
            None
        } else {
            Some(dbfs)
        }
    }

    /// Returns the volume between 0..=1 by going through dBFS scaling.
    pub fn volume(&self) -> f32 {
        match self.dbfs() {
            Some(dbfs) => dbfs.remap(self.settings.threshold, 0., 0., 1.),
            None => 0.,
        }
    }
}

#[derive(Resource, SettingsGroup, Reflect)]
#[reflect(Resource, SettingsGroup, Default)]
#[settings_group(file = "volume")]
pub struct VolumeSettings {
    threshold: f32,
}
impl Default for VolumeSettings {
    fn default() -> Self {
        Self { threshold: -32. }
    }
}

#[derive(Resource)]
pub struct VolumeInput {
    volume: Arc<AtomicU16>,
}
impl VolumeInput {
    pub fn new() -> Self {
        let volume = Arc::new(AtomicU16::new(0));
        let volume_thread = Arc::clone(&volume);

        std::thread::spawn(move || {
            let host = cpal::available_hosts()
                .into_iter()
                .find(|id| *id == cpal::HostId::PipeWire)
                .and_then(|id| cpal::host_from_id(id).ok())
                .unwrap_or(cpal::default_host());

            let device = host.default_input_device().unwrap();
            let input_config = device.default_input_config().unwrap();
            let stream_config = StreamConfig {
                channels: 1,
                sample_rate: input_config.sample_rate(),
                buffer_size: BufferSize::Fixed(1000),
            };
            let build_result = device.build_input_stream(
                stream_config,
                move |data: &[i16], _info| {
                    let mut data: Vec<u16> = data.iter().map(|x| x.unsigned_abs()).collect();
                    let len = data.len();
                    let (_, vol, _) = data.select_nth_unstable(len / 20 * 19);
                    volume_thread
                        .store(vol.saturating_mul(2), std::sync::atomic::Ordering::Relaxed);
                },
                |err| error!("stream error: {err}"),
                Some(Duration::from_secs(3)),
            );
            match build_result {
                Ok(stream) => {
                    if let Ok(()) = stream.play() {
                        loop {
                            std::thread::sleep(Duration::from_millis(30))
                        }
                    } else {
                        error!("failed to play stream");
                    }
                }
                Err(err) => error!("failed to build stream: {err}"),
            }
        });

        Self { volume }
    }

    /// Gets the current volume as a value between [0,1]
    pub fn raw_volume(&self) -> f32 {
        self.volume.load(Ordering::Relaxed) as f32 / u16::MAX as f32
    }
}
