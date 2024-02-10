use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Serialize, Deserialize)]
pub struct Movies {
    pub id: Uuid,
    pub title: String,
    pub description: String,
    pub genre: Option<Vec<String>>,
    pub actors: Option<Vec<String>>
}