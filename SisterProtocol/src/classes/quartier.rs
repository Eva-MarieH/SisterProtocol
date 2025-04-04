use crate::classes::cachette::Cachette;
use crate::classes::entites::{Garde, Ordinateur, Serveur, Marchand, PNJ};
use crate::classes::boss::Boss;

pub struct Quartier {
    pub couleur: String,
    pub cachettes: Vec<Cachette>,
    pub gardes: Vec<Garde>,
    pub ordinateurs: Vec<Ordinateur>,
    pub serveur: Serveur,
    pub boss: Option<Boss>,
    pub marchands: Vec<Marchand>,
    pub pnjs: Vec<PNJ>,
}