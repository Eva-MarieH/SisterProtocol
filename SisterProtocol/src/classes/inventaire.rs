use serde::{Deserialize, Serialize};

#[derive(Deserialize, Debug, Clone, Serialize)]
pub enum TypeObjet {
    Nourriture,
    Amelioration,
    CarteAcces,
}

impl PartialEq for TypeObjet {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (TypeObjet::Nourriture, TypeObjet::Nourriture) => true,
            (TypeObjet::Amelioration, TypeObjet::Amelioration) => true,
            (TypeObjet::CarteAcces, TypeObjet::CarteAcces) => true,
            _ => false,
        }
    }
}

#[derive(Deserialize, Debug, Clone, Serialize)]
pub struct Objet {
    pub id: u8,
    pub nom: String,
    pub effet: i32,
    pub type_objet: TypeObjet,
    pub prix: Option<i32>,  // Prix optionnel
}

#[derive(Deserialize, Debug, Clone, Serialize)]
pub struct Inventaire {
    pub objets: Vec<ObjetQuantifie>,
}

#[derive(Deserialize, Clone, Debug, Serialize)]
pub struct ObjetQuantifie {
    pub id: u8,
    pub quantity: u32,
}
