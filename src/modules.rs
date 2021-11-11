pub use async_trait::async_trait;
pub use hidden_modules_stuff::*;
pub use twilight_gateway::Event::*;

mod hidden_modules_stuff{
	use std::error::Error;
	use std::sync::Arc;
	use twilight_gateway::Event as GatewayEvent;
	use twilight_gateway::Intents;
	use twilight_http::Client as HttpClient;
	use twilight_model::user::CurrentUser;

	pub type InitResult<T = ()> = Result<T, Box<dyn Error + Send + Sync>>;
	pub type HandleResult<T = ()> = Result<T, Box<dyn Error + Send + Sync>>;

	pub struct InitStuff {
		pub current_user: CurrentUser,
		pub http: Arc<HttpClient>
	}

	pub struct Event {
		pub shard_id: u64,
		pub event: GatewayEvent,
		pub http: Arc<HttpClient>
	}

	#[async_trait::async_trait]
	pub trait Module: Send + Sync {
		#[inline]
		fn intents(&self) -> Intents {
			Intents::empty()
		}

		async fn init(&mut self, _: &InitStuff) -> InitResult { Ok(()) }
		async fn handle_event(&self, event: Event) -> HandleResult;
	}
}
