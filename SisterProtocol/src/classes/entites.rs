use crate::classes::inventaire::Objet;

pub struct Serveur {
    pub id: u8,
    pub pass: i32,
    pub loot: Vec<Objet>,
}

pub struct Ordinateur {
    pub id: u8,
    pub point_hack: i32,
    pub loot: Vec<Objet>,
}