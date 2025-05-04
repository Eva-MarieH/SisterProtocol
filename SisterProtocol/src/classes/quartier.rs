use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize,Serialize)]
pub struct Quartier{
    pub color: String,
    pub enemies: Vec<u8>,
    pub merchant: u8,
    pub residents: Vec<u8>,
    pub ordinateurs: Option<Vec<u8>>,
    pub server: Option<u8>,
    pub boss: Option<u8>,
}

