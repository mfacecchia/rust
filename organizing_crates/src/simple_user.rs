// NOTE: Importing stuff from the other namespace.
// Importing using absolute import (root starts with `crate::`, following with the path to the required resource)
// Of course, only public imports can be imported
use crate::developer::fetch::{FetchStatus,RequestMethod,Fetch};

pub fn get_favourite_songs() -> FetchStatus {
    let mut fetch_options = Fetch::build_fetch("https://provider.com/api/v1/me/favourite-songs", RequestMethod::GET);
    return fetch_options.make_request();
}