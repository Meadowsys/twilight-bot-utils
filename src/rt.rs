use std::time::Duration;
use tokio::runtime::Builder;
use tokio::runtime::Runtime;
use crate::env::Env;

pub fn make_tokio_runtime() -> Runtime {
	let rt = Builder::new_multi_thread()
		.enable_all()
		.worker_threads(Env::num_threads())
		.thread_keep_alive(Duration::from_secs(60))
		.build()
		.unwrap();

	rt
}

/// Does the same as [`make_tokio_runtme`] except it creates one using
/// the current thread, which is more ideal for development environments
/// where you don't need literally all of your computer's power
///
/// [`make_tokio_runtme`]: crate::rt::make_tokio_runtime
pub fn make_cheap_tokio_runtime() -> Runtime {
	let rt = Builder::new_current_thread()
		.enable_all()
		.thread_keep_alive(Duration::from_secs(60))
		.build()
		.unwrap();

	rt
}
