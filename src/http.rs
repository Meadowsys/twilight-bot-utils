use crate::env::Env;
use crate::MainResult;
use std::sync::Arc;
use twilight_http::Client as HttpClient;

pub fn setup_http(env: &Env) -> MainResult<Arc<HttpClient>> {
	let http = HttpClient::new(env.token().into());
	let http = Arc::new(http);

	Ok(http)
}
