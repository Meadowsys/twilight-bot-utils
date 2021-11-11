pub mod modules;
pub mod env;

use env::Env;
use std::error::Error;
use std::sync::Arc;
use tokio::runtime::Runtime;
use twilight_gateway::Cluster;
use twilight_gateway::cluster::Events;
use twilight_gateway::Intents;
use twilight_http::Client as HttpClient;

pub type MainResult<T = ()> = Result<T, Box<dyn Error + Send + Sync>>;

pub fn make_tokio_runtime() -> Runtime {
	let rt = tokio::runtime::Builder::new_multi_thread()
		.enable_all()
		.worker_threads(2)
		.max_blocking_threads(32)
		.thread_keep_alive(std::time::Duration::from_secs(60))
		.build()
		.unwrap();

	rt
}

pub fn setup_http(env: &Env) -> MainResult<Arc<HttpClient>> {
	let http = HttpClient::new(env.token().into());
	let http = Arc::new(http);

	Ok(http)
}

pub async fn setup_cluster(env: &Env, intents: &Intents) -> MainResult<(Arc<Cluster>, Events)> {
	let (cluster, events) = Cluster::builder(env.token(), intents.clone())
		.shard_scheme(twilight_gateway::cluster::ShardScheme::Auto)
		.build()
		.await?;
	let cluster = Arc::new(cluster);

	Ok((cluster, events))
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

	// useful types
	pub use crate::MainResult;
}
