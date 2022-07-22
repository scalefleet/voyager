mod authentication;
mod database;
mod organization;

pub use authentication::*;
pub use database::*;
pub use organization::*;

#[derive(Debug, serde::Deserialize)]
pub struct List<T> {
    #[serde(rename = "type")]
    _type: String,
    pub current_page: usize,
    pub next_page: Option<usize>,
    pub next_page_url: Option<String>,
    pub prev_page: Option<usize>,
    pub prev_page_url: Option<String>,
    pub data: Vec<T>,
}
