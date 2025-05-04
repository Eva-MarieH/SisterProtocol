use serde::{Deserialize, Serialize};

#[derive(Deserialize, Debug, Clone,Serialize )]
pub enum TypeObjet {
    Nourriture,
    Amelioration,
    CarteAcces,
}

#[derive(Deserialize, Debug, Clone,Serialize)]
pub struct Objet {
    pub id: u8,
    pub nom: String,
    pub effet: i32,
    pub type_objet: TypeObjet,
    pub prix: i32,
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
