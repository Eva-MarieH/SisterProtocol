use rand::Rng;

pub struct Combat {
    pub force_hero: i32,
    pub force_ennemi: i32,
    pub vie_hero: i32,
    pub vie_ennemi: i32,
}

impl Combat {
    pub fn lancer(&mut self) -> (i32, i32) {
        let mut rng = rand::thread_rng();

        // Dégâts infligés 
        let mut x_hero: i32;
    let mut x_ennemi: i32;
    if self.force_hero > self.force_ennemi {
            x_hero = rng.gen_range(1..=5) * 20;
        x_ennemi =  rng.gen_range(1..=20) * 5;
        } else if self.force_hero < self.force_ennemi {
        x_hero = rng.gen_range(1..=20) * 5;
        x_ennemi =  rng.gen_range(1..=5) * 20;
            rng.gen_range(1..=10) * 10
        } else {
            x_hero = rng.gen_range(1..=10) * 10;
        x_ennemi =  rng.gen_range(1..=10) * 10;
        };

        // Mise à jour des vies
        self.vie_hero -= x_ennemi;
        self.vie_ennemi -= x_hero;

        println!("Tu infliges {} points de degats, l'ennemi a maintenant {} points de vie", x_hero);
        println!("L'ennemi inflige {} points de degats, il te reste {} points de vie", x_ennemi, self.vie_hero);

        (degats_hero, degats_ennemi)
    }
}
