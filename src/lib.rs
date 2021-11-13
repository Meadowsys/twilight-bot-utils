pub mod cluster;
pub mod env;
pub mod http;
pub mod modules;
pub mod rt;
pub mod run;

use std::error::Error;

pub type MainResult<T = ()> = Result<T, Box<dyn Error + Send + Sync>>;

pub mod prelude {
	// useful structs from this lib
	pub use crate::env::Env;
	pub use crate::modules::Event;
	pub use crate::modules::InitStuff;

	// useful structs and types from elsewhere
	pub use std::sync::Arc;
	pub use std::sync::Mutex;
	pub use twilight_gateway::cluster::Cluster;
	pub use twilight_gateway::cluster::Events;
	pub use twilight_gateway::cluster::ShardScheme;
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

/// re-exports all these dependencies so that they don't have to be declared
/// as a dependency in consumer code, and so that you can be sure that
/// the version used in this lib is the same as the one you are using in your code
pub mod deps {
	pub use async_trait;
	pub use chrono;
	pub use dotenv;
	pub use futures;
	pub use tokio;
	pub use twilight_cache_inmemory;
	pub use twilight_gateway;
	pub use twilight_http;
	pub use twilight_mention;
	pub use twilight_model;
	pub use twilight_standby;
	pub use twilight_util;
}
