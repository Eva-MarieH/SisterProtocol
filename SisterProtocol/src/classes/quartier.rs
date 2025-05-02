use crate::classes::cachette::Cachette;
use crate::classes::entites::{Ordinateur, Serveur};
use crate::classes::personnage::{Garde, Marchand, PNJ, Boss};

pub struct Quartier {
    pub couleur: String,
    pub gardes: Vec<Garde>,
    pub ordinateurs: Option<Vec<Ordinateur>>,
    pub serveur: Option<Serveur>,
    pub boss: Option<Boss>,
    pub marchand: Marchand,
    pub pnjs: Vec<PNJ>,
}