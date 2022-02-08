use tokio::runtime::Runtime;
use crate::env::Env;

pub fn make_tokio_runtime() -> Runtime {
	let rt = tokio::runtime::Builder::new_multi_thread()
		.enable_all()
		.worker_threads(*Env::num_threads())
		.thread_keep_alive(std::time::Duration::from_secs(60))
		.build()
		.unwrap();

	rt
}
