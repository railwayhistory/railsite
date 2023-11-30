use std::sync::Arc;
use crate::route;
use crate::config::Config;
use crate::state::{RequestState, ServerState};

pub async fn serve(config: &Config, state: Arc<ServerState>) {
    httools::server::serve(config.listen, state, |state, request| async move {
        match RequestState::from_request(&request, state.clone()) {
            Ok(state) => Ok(route::Root::process(request, &state)),
            Err(response) => Ok(response)
        }
    }).await
}


