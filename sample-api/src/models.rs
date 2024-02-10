use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Recipe {
    pub id: u32,
    pub title: String,
    pub description: String,
    pub tag: String,
    pub create_at: String,
    pub update_ad: String,
    pub done: bool,
}
