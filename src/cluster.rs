use crate::env::Env;
use crate::MainResult;
use std::sync::Arc;
use twilight_gateway::Cluster;
use twilight_gateway::cluster::Events;
use twilight_gateway::Intents;

pub async fn setup_cluster(env: &Env, intents: &Intents) -> MainResult<(Arc<Cluster>, Events)> {
	let (cluster, events) = Cluster::builder(env.token().into(), *intents)
		.shard_scheme(twilight_gateway::cluster::ShardScheme::Auto)
		.build()
		.await?;
	let cluster = Arc::new(cluster);

	Ok((cluster, events))
}
