use bevy::prelude::*;
use std::{path::PathBuf, time::Duration};

use notify_debouncer_full::{
    DebounceEventResult, DebouncedEvent, Debouncer, RecommendedCache, new_debouncer,
    notify::{RecommendedWatcher, RecursiveMode},
};

#[must_use = "If dropped, this will stop running."]
pub fn watch_file<T>(
    file: PathBuf,
    mut on_event: T,
) -> Option<Debouncer<RecommendedWatcher, RecommendedCache>>
where
    T: FnMut(&DebouncedEvent) + Send + 'static,
{
    info!("setting up");
    let debouncer = new_debouncer(
        Duration::from_secs(1),
        None,
        move |result: DebounceEventResult| {
            info!("got stuff");
            match result {
                Err(errors) => errors.iter().for_each(|error| {
                    error!("Encountered a filesystem watcher error {error:?}");
                }),
                Ok(events) => {
                    for event in events.iter() {
                        on_event(event)
                    }
                }
            }
        },
    );
    let mut debouncer = match debouncer {
        Ok(d) => d,
        Err(e) => {
            error!("Error starting debouncer: {e:?}");
            return None;
        }
    };

    let _ = debouncer
        .watch(file, RecursiveMode::Recursive)
        .inspect_err(|e| error!("Error watching file: {e:?}"))
        .inspect(|_| {
            info!("set it up");
        });
    Some(debouncer)
}
