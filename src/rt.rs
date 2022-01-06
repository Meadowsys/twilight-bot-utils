use tokio::runtime::Runtime;

pub fn make_tokio_runtime() -> Runtime {
	let rt = tokio::runtime::Builder::new_multi_thread()
		.enable_all()
		.worker_threads(2)
		.thread_keep_alive(std::time::Duration::from_secs(60))
		.build()
		.unwrap();

	rt
}
