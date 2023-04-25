pub mod service;
include!(concat!(env!("OUT_DIR"), "/_.rs"));

pub struct AppContext {
    pub hardcoded_database: Vec<Book>,
}
