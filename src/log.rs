use std::num::NonZeroU8;

use tracing::level_filters::LevelFilter;
use tracing_subscriber::{EnvFilter, Layer, layer::SubscriberExt, util::SubscriberInitExt};

pub fn init_logging(verbosity: Option<NonZeroU8>) {
    #[cfg(not(debug_assertions))]
    const DEFAULT_LOG_LEVEL: &str = "pngtuber=warn,error";
    #[cfg(debug_assertions)]
    const DEFAULT_LOG_LEVEL: &str = "pngtuber=trace,info";
    const ENV: &str = "PNGTUBER_LEVEL";

    let registry = tracing_subscriber::registry().with(tracing_error::ErrorLayer::default());
    let subscriber = tracing_subscriber::fmt::layer()
        .pretty()
        .with_line_number(true);
    let level = match verbosity.map(|x| x.into()) {
        None => {
            let envfilter = EnvFilter::try_from_env(ENV).unwrap_or_else(|_| {
                EnvFilter::builder()
                    .parse(DEFAULT_LOG_LEVEL)
                    .expect("default filter should be valid")
            });
            registry.with(subscriber.with_filter(envfilter)).init();
            return;
        }
        Some(1) => LevelFilter::INFO,
        Some(2) => LevelFilter::DEBUG,
        Some(3) => LevelFilter::TRACE,
        _ => panic!("too verbose!!!!"),
    };
    registry.with(subscriber.with_filter(level)).init();
}
