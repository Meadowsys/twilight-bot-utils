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

pub async fn watch_for_shutdown_signals(f: impl FnOnce(&str)) {
	use tokio::signal::unix::{ signal, SignalKind };

	let mut sigint = signal(SignalKind::interrupt()).unwrap();
	let mut sigterm = signal(SignalKind::terminate()).unwrap();

	tokio::select! {
		// without biased, tokio::select! will choose random branches to poll,
		// which incurs a small cpu cost for the random number generator
		// biased polling is fine here
		biased;

		_ = sigint.recv() => {
			f("SIGINT");
		}
		_ = sigterm.recv() => {
			f("SIGTERM")
		}
	}
}
