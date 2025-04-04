pub struct Combat {
    pub force_hero: i32,
    pub force_ennemi: i32,
}

impl Combat {
    pub fn lancer(&self) -> bool {
        self.force_hero > self.force_ennemi
    }
}