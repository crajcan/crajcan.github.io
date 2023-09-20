use serde::Deserialize;

#[derive(Clone, PartialEq, Deserialize)]
pub(crate) struct Video {
    pub id: usize,
    pub title: String,
    pub speaker: String,
    pub url: String,
}