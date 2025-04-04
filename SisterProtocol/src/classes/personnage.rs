pub struct Personnage {
    pub nom: String,
}

pub struct Hero {
    pub vie: i32,
    pub force: i32,
    pub intelligence: i32,
    pub argent: i32,
    pub position: String,
    pub inventaire: Inventaire,
    pub amelioration: Option<Amelioration>,
}

impl Hero {
    pub fn perdre_vie(&mut self, degats: i32) {
        self.vie -= degats;
    }
}