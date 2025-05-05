use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize,Serialize)]
pub struct Quartier{
    pub color: String,
    pub guards: Option<Vec<u8>>,
    pub merchant: u8,
    pub residents: Vec<u8>,
    pub ordinateurs: Option<Vec<u8>>,
    pub server: Option<u8>,
    pub boss: bool,
}

