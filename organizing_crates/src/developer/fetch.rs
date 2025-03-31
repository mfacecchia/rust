pub enum FetchStatus {
    SUCCESS,
    FAILURE,
}

pub enum RequestMethod {
    GET,
    POST,
    PUT,
    PATCH,
    DELETE,
}

// NOTE: Even if the struct is public, all the fields are private by default,
// set them to public using the same `pub` keyword
pub struct Fetch {
    method: RequestMethod,
    url: String,
    status: Option<FetchStatus>,
    fetch_id: String
}

impl Fetch {
    pub fn build_fetch(url: &str, method: RequestMethod) -> Fetch {
        return Fetch {
            url: String::from(url),
            method,
            fetch_id: String::from(""),
            status: None
        }
    }
    pub fn make_request(&mut self) -> FetchStatus {
        self.fetch_id = String::from("abc123");
        println!("Making request");
        return FetchStatus::SUCCESS;
    }
}