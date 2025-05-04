use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct Quartier{
    pub color: String,
    //pub hidingPlace: Vec<CachetteBrute>,
    pub enemies: Vec<u8>,
    pub merchant: u8,
    pub residents: Vec<u8>,
    pub ordinateurs: Option<Vec<u8>>,
    pub server: Option<u8>,
    pub boss: Option<u8>,
}

