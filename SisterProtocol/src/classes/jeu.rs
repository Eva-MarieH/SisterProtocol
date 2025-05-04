use crate::classes::quartier::Quartier;
use crate::classes::personnage::Hero;

pub struct Jeu {
    pub quartiers: Vec<Quartier>,
    pub quartier_actuel: String,
    pub hero : Hero,
}
impl Jeu {
    pub fn peut_pirater(&self) -> bool {
        self.quartiers
            .iter()
            .find(|q| q.color == self.quartier_actuel)
            .map(|q| q.guards.is_empty())
            .unwrap_or(false) // Si le quartier n'est pas trouvé, mieux vaut interdire le piratage
    }
}
