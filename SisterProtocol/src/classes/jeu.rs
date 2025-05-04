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
            .map(|q| match &q.guards {
                Some(gardes) => gardes.is_empty(),
                None => true, 
            })
            .unwrap_or(false)
    }
    pub fn peut_combattre(&self) -> bool {
        if self.hero.vie > 0 {
            true
        } else {
            false
        }
    }

}
