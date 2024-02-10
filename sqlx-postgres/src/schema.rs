use serde::{Deserialize, Serialize};

#[derive(Deserialize, Debug)]
pub struct FilterOptions {
    pub page: Option<usize>,
    pub limit: Option<usize>
}

#[derive(Deserialize, Debug)]
pub struct ParamOptions {
    pub id: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct CreateMovieSchema {
    pub title: String,
    pub description: String,
    pub genre: Vec<String>,
    pub actors: Vec<String>,
}
#[derive(Serialize, Deserialize, Debug)]
pub struct UpdateMovieSchema {
    pub title: Option<String>,
    pub description: Option<String>,
    pub genre: Option<Vec<String>>,
    pub actors: Option<Vec<String>>,
}
