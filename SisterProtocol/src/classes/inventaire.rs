pub struct Inventaire {
    pub objets: Vec<Objet>,
}

pub enum TypeObjet {
    Nourriture,
    Amelioration,
    CarteAcces,
}

pub struct Objet {
    pub id: u8,
    pub nom: String,
    pub effet: i32,
    pub type_objet: TypeObjet,
}

pub struct Upgrade {
    pub niveau: i32,
}