pub use async_trait::async_trait;
pub use twilight_gateway::Event::*;

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

#[async_trait]
pub trait Module: Send + Sync {
	#[inline]
	fn intents(&self) -> Intents {
		Intents::empty()
	}

	async fn init(&mut self, _: &InitStuff) -> InitResult { Ok(()) }
	async fn handle_event(&self, event: Event) -> HandleResult;
}

pub type ProperWrappedModules = Arc<Vec<Arc<Box<dyn Module>>>>;

pub struct ModuleHandler {
	modules: Vec<Box<dyn Module>>
}

impl ModuleHandler {
	pub fn new() -> ModuleHandler {
		ModuleHandler { modules: vec![] }
	}

	pub fn with_modules(modules: Vec<Box<dyn Module>>) -> ModuleHandler {
		ModuleHandler { modules }
	}

	pub fn add_module(mut self, module: impl Module + 'static) -> ModuleHandler {
		self.modules.push(Box::new(module));
		self
	}

	pub fn add_modules(mut self, modules: Vec<Box<dyn Module>>) -> ModuleHandler {
		self.modules.extend(modules);
		self
	}

	pub async fn init_modules(mut self, current_user: CurrentUser, http: Arc<HttpClient>) -> crate::MainResult<ModuleHandler> {
		let init_stuff = InitStuff { current_user, http };

		for module in self.modules.iter_mut() {
			module.init(&init_stuff).await?;
		}

		Ok(self)
	}

	pub fn into_modules(self) -> ProperWrappedModules {
		Arc::new(
			self.modules.into_iter()
				.map(Arc::new)
				.collect()
		)
	}
}
