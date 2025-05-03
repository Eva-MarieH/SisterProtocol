use crate::classes::cachette::Cachette;
use crate::classes::entites::{Ordinateur, Serveur};
use crate::classes::personnage::{Garde, Marchand, Resident, Boss};
use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct Quartier{
    pub color: String,
    //pub hidingPlace: Vec<CachetteBrute>,
    pub unlocked: bool,
    pub enemies: Vec<usize>,
    pub merchant: usize,
    pub residents: Vec<usize>,
    pub pc: Option<Vec<usize>>,
    pub server: Option<usize>,
    pub boss: Option<usize>,
}

