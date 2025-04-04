pub struct Inventaire {
    pub objets: Vec<Objet>,
}

pub struct Objet {
    pub nom: String,
    pub taille: i32,
    pub poids: i32,
    pub valeur: i32,
}