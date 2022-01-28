use serde::de::DeserializeOwned;
use warp::Filter;

mod error;
mod state;
use crate::config;

pub mod routes;
pub use error::handle_error;
pub use state::{with_state, AppState};

pub fn json_body<T: DeserializeOwned + Send>(
) -> impl Filter<Extract = (T,), Error = warp::Rejection> + Clone {
    warp::body::content_length_limit(config::MAX_RESPONSE_SIZE).and(warp::body::json())
}
