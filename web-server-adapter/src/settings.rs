use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Debug)]
pub struct Settings {
    pub database: Database,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct Database {
    pub url: String,
}
