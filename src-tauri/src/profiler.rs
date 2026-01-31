#[cfg(feature = "profiling")]
use once_cell::sync::Lazy;
#[cfg(feature = "profiling")]
use std::fs::File;
#[cfg(feature = "profiling")]
use std::io::BufWriter;
#[cfg(feature = "profiling")]
use tracing::instrument;
#[cfg(feature = "profiling")]
use tracing_flame::FlameLayer;
#[cfg(feature = "profiling")]
use tracing_flame::FlushGuard;
#[cfg(feature = "profiling")]
use tracing_subscriber::prelude::*;
#[cfg(feature = "profiling")]
use tracing_subscriber::registry::Registry;

#[cfg(feature = "profiling")]
static PROFILER: Lazy<std::sync::Mutex<Option<FlushGuard<BufWriter<File>>>>> =
    Lazy::new(|| std::sync::Mutex::new(None));

#[cfg(feature = "profiling")]
pub fn setup_global_subscriber() {
    let fmt_layer = tracing_subscriber::fmt::Layer::default();
    let (flame_layer, guard) = tracing_flame::FlameLayer::with_file("./trace.folded").unwrap();

    let subscriber = tracing_subscriber::registry::Registry::default()
        .with(fmt_layer)
        .with(flame_layer);

    tracing::subscriber::set_global_default(subscriber).expect("Could not set global default");

    *PROFILER.lock().unwrap() = Some(guard);
}

#[cfg(feature = "profiling")]
pub fn drop_global_subscriber() {
    if let Some(guard) = PROFILER.lock().unwrap().take() {
        drop(guard);
    }
}