pub mod cluster;
pub mod env;
pub mod http;
pub mod modules;
pub mod rt;
pub mod run;

use std::error::Error;

pub type MainResult<T = ()> = Result<T, Box<dyn Error + Send + Sync>>;

pub mod prelude {
	pub use crate::cluster::setup_cluster;
	pub use crate::deps::*;
	pub use crate::env::Env;
	pub use crate::http::get_current_user;
	pub use crate::http::setup_http;
	pub use crate::MainResult;
	pub use crate::modules::*;
	pub use crate::rt::make_tokio_runtime;
	pub use crate::run::process_events;
	pub use crate::run::watch_for_shutdown_signals;
	pub use futures::future::Future;
	pub use futures::future::FutureExt;
	pub use futures::stream::StreamExt;
	pub use std::error::Error;
	pub use std::sync::Arc;
	pub use std::sync::Mutex;
	pub use tokio::spawn;
	pub use twilight_gateway::cluster::Cluster;
	pub use twilight_gateway::cluster::Events;
	pub use twilight_gateway::cluster::ShardScheme;
	pub use twilight_gateway::Intents;
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
