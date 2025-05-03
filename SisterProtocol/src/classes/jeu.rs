use crate::classes::quartier::Quartier;
use crate::classes::personnage::Hero;

pub struct Jeu {
    pub quartiers: Vec<Quartier>,
    pub quartier_actuel: String,
    pub hero : Hero,
}