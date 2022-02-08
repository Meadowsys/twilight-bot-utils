#[macro_use]
extern crate lazy_static;

pub mod cluster;
pub mod env;
pub mod http;
pub mod modules;
pub mod rt;
pub mod run;

use std::error::Error;

pub type MainResult<T = ()> = Result<T, Box<dyn Error + Send + Sync>>;

pub mod prelude;

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
	pub use twilight_embed_builder;
	pub use twilight_gateway;
	pub use twilight_http;
	pub use twilight_mention;
	pub use twilight_model;
	pub use twilight_standby;
	pub use twilight_util;
}
