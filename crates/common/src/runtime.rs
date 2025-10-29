use std::sync::OnceLock;

use tokio::runtime::Builder;

static RUNTIME: OnceLock<tokio::runtime::Runtime> = OnceLock::new();

const ANT_WORKER_THREADS: &str = "ANT_WORKER_THREADS";

const DEFAULT_OS_THREADS: usize = 0;

fn initialize_runtime() -> tokio::runtime::Runtime{
    let worker_threads = std::env::var(ANT_WORKER_THREADS)
        .ok()
        .and_then(|val| val.parse::<usize>().ok())
        .unwrap_or(DEFAULT_OS_THREADS);

    let mut builder = Builder::new_multi_thread();

    let builder = if worker_threads > 0{
        builder.worker_threads(worker_threads)
    } else {
        &mut builder
    };
    builder
        .enable_all()
        .build()
        .expect("Faild to create tokio runtime")
}

pub fn get_runtime() -> &'static tokio::runtime::Runtime {
    RUNTIME.get_or_init(initialize_runtime)
}
