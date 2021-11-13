use crate::env::Env;
use crate::MainResult;
use futures::Future;
use std::sync::Arc;
use twilight_http::Client as HttpClient;
use twilight_model::user::CurrentUser;

pub fn setup_http(env: &Env) -> MainResult<Arc<HttpClient>> {
	let http = HttpClient::new(env.token().into());
	let http = Arc::new(http);

	Ok(http)
}

pub fn get_current_user(http: &Arc<HttpClient>) -> impl Future<Output = MainResult<CurrentUser>> {
	let http = Arc::clone(http);

	async move {
		let current_user = http.current_user()
			.exec().await?
			.model().await?;

		Ok(current_user)
	}
}
