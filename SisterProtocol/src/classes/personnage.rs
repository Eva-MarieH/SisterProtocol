use crate::classes::inventaire::Objet;
use crate::classes::inventaire::Inventaire;
use crate::classes::inventaire::Upgrade;

pub struct Hero {
    pub nom: String,
    pub vie: i32,
    pub force: i32,
    pub intelligence: i32,
    pub argent: i32,
    pub position: String,
    pub inventaire: Inventaire,
    pub amelioration: Option<Upgrade>,
}

impl Hero {
    pub fn perdre_vie(&mut self, degats: i32) {
        self.vie -= degats;
    }
}

pub struct Garde {
    pub id: u8,
    pub nom: String,
    pub vie: i32,
    pub force: i32,
    pub loot: Vec<Objet>,
}

pub struct Marchand {
    pub id: u8,
    pub nom: String,
    pub stock: Vec<Objet>,
}

pub struct PNJ {
    pub id: u8,
    pub nom: String,
    pub message: String,
}

pub struct Boss {
    pub nom: String,
    pub vie: i32,
    pub force: i32,
}