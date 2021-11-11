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

pub mod prelude {
	// useful structs from this lib
	pub use crate::env::Env;
	pub use crate::modules::Event;
	pub use crate::modules::InitStuff;

	// useful structs and types from elsewhere
	pub use std::sync::Arc;
	pub use twilight_gateway::cluster::Cluster;
	pub use twilight_gateway::cluster::Events;
	pub use twilight_gateway::cluster::ShardScheme::Auto;
	pub use twilight_gateway::Intents;

	// useful traits
	pub use futures::future::Future;
	pub use futures::future::FutureExt;
	pub use futures::stream::StreamExt;
	pub use std::error::Error;

	// useful functions
	pub use tokio::spawn;
}
