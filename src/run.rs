use crate::modules::Event;
use futures::StreamExt;
use std::sync::Arc;
use tokio::spawn;
use twilight_gateway::cluster::Events;
use twilight_http::Client as HttpClient;

pub async fn process_events(
	mut events: Events,
	http: Arc<HttpClient>,
	modules: crate::modules::ProperWrappedModules
) {
	while let Some((shard_id, event)) = events.next().await {
		let modules = Arc::clone(&modules);
		let event = event.clone();
		let http = Arc::clone(&http);

		spawn(async move {
			for module in modules.iter() {
				let module = Arc::clone(module);
				let event = event.clone();
				let http = Arc::clone(&http);
				let event = Event { shard_id, event, http };

				spawn(async move { module.handle_event(event).await });
			}
		});
	}
}
