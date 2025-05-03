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
<<<<<<< HEAD
}
=======
}

>>>>>>> 63e707aa6aa897068c5df3f9e24d66c5371d239a
