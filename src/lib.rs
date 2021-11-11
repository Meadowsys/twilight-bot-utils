pub mod modules;
pub mod env;

pub fn make_tokio_runtime() -> tokio::runtime::Runtime {
	let rt = tokio::runtime::Builder::new_multi_thread()
		.enable_all()
		.worker_threads(2)
		.max_blocking_threads(32)
		.thread_keep_alive(std::time::Duration::from_secs(60))
		.build()
		.unwrap();

	rt
}
