mod audio;

pub use self::audio::*;

use warp::Filter;

pub fn get_routes() -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    audio_route()
}

