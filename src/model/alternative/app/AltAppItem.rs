use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize)]
pub struct AltApp {
    pub id: String,
    pub name: String
}

