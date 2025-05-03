use serde::Deserialize;

#[derive(Deserialize, Debug, Clone)]
pub enum TypeObjet {
    Nourriture,
    Amelioration,
    CarteAcces,
}

#[derive(Deserialize, Debug, Clone)]
pub struct Objet {
    pub id: u8,
    pub nom: String,
    pub effet: i32,
    pub type_objet: TypeObjet,
    pub prix: i32,
}

#[derive(Deserialize)]
pub struct Inventaire {
    pub objets: Vec<ObjetQuantifie>,
}

#[derive(Deserialize, Clone)]
pub struct ObjetQuantifie {
    pub id: u8,
    pub quantity: u32,
}
