use rand::{Rng,rng};
use std::io::{self, Write};

use crate::classes::jeu::Jeu;
use crate::utils::{save, utilisation_objet};
use crate::utils::ini;

use super::quartier::Quartier;

pub struct Combat {
    pub force_hero: i32,
    pub force_ennemi: i32,
    pub vie_hero: i32,
    pub vie_ennemi: i32,
}

impl Combat {
    pub fn lancer(&mut self) -> (i32, i32) {
        let mut rng_gen = rng();

        // Dégâts infligés 
        let x_hero: i32;
        let x_ennemi: i32;
        if self.force_hero > self.force_ennemi {
            x_hero = rng_gen.random_range(1..=5) * 20;
            x_ennemi =  rng_gen.random_range(1..=20) * 5;
        } 
        else if self.force_hero < self.force_ennemi {
            x_hero = rng_gen.random_range(1..=20) * 5;
            x_ennemi =  rng_gen.random_range(1..=5) * 20;
        } 
        else {
            x_hero = rng_gen.random_range(1..=10) * 10;
            x_ennemi =  rng_gen.random_range(1..=10) * 10;
        };

        // Mise à jour des vies 
        self.vie_ennemi -= x_hero;
        println!("Tu infliges {} points de degats, l'ennemi a maintenant {} points de vie", x_hero, self.vie_ennemi);
        if self.vie_ennemi <= 0 {
            return (self.vie_hero, self.vie_ennemi)
        }

        self.vie_hero -= x_ennemi;
        println!("L'ennemi inflige {} points de degats, il te reste {} points de vie", x_ennemi, self.vie_hero);
        if self.vie_hero <= 0{
            return (self.vie_hero, self.vie_ennemi)
        }

        (self.vie_hero, self.vie_ennemi)
    }

    pub fn supprimer_garde(quartier_actuel: &mut Quartier) {
        if let Some(gardes) = &mut quartier_actuel.guards {
            if gardes.is_empty() {
                quartier_actuel.ordinateurs = None;
            }
            else{
                gardes.remove(0);
            }
        }
    }

    pub fn lancer_combat(jeu: &mut Jeu) {
        // Récupération des infos nécessaires (copiées ou clonées) avant la boucle
        let mut combat: Combat;
        let garde_nom: String;
        let quartier_couleur = jeu.quartier_actuel.clone();
    
        {
            let quartier_actuel = jeu.quartiers.iter_mut()
                .find(|q| q.color == quartier_couleur)
                .expect("Quartier actuel introuvable");
    
            let garde = ini::charger_premier_garde_quartier(quartier_actuel)
                .expect("Aucun garde trouvé");
    
            garde_nom = garde.name.clone();
    
            combat = Combat {
                force_hero: jeu.hero.force,
                force_ennemi: garde.force,
                vie_hero: jeu.hero.vie,
                vie_ennemi: 100,
            };
    
            println!("\nTu rencontres {} !", garde_nom);
        }
    
        loop {
            println!("\nQue veux-tu faire ?");
            println!("1. Attaquer");
            println!("2. Utiliser un objet");
            println!("3. Abandonner");
    
            print!("> ");
            io::stdout().flush().unwrap();
            let mut choix = String::new();
            io::stdin().read_line(&mut choix).unwrap();
    
            match choix.trim() {
                "1" => {
                    let (vie_hero, vie_ennemi) = combat.lancer();
                    combat.vie_hero = vie_hero;
                    combat.vie_ennemi = vie_ennemi;
                    jeu.hero.vie = vie_hero;
    
                    if vie_ennemi <= 0 {
                        println!("Tu as vaincu {}!", garde_nom);

                        // Gagner de l'argent
                        let mut rng_argent = rng();
                        let argent_gagne = rng_argent.random_range(10.. 50);
                        println!("Tu as gagné {} crédits !", argent_gagne);
                        jeu.hero.argent += argent_gagne;

                        // Re-emprunter maintenant que l'ancien est libéré
                        let quartier = jeu.quartiers.iter_mut()
                            .find(|q| q.color == quartier_couleur)
                            .expect("Quartier introuvable");
                        Combat::supprimer_garde(quartier);

                        break;
                    } else if vie_hero <= 0 {
                        println!("Tu as perdu...");
                        println!("Vous vous évanouissez.");
                        jeu.hero.vie = 10;
                        break;
                    }
                },
                "2" => {
                    utilisation_objet::utilisation_objet(jeu);
                },
                "3" => {
                    println!("Tu abandonnes le combat.");
                    jeu.hero.vie = combat.vie_hero;
                    break;
                },
                _ => println!("Choix invalide."),
            }
        }
    
        // Sauvegarde finale
        save::enregistrer_hero(&jeu.hero);
    }


    pub fn lancer_combat_boss(jeu: &mut Jeu) {
        // Récupération des infos nécessaires (copiées ou clonées) avant la boucle
        let mut combat: Combat;

        let boss = ini::charger_boss_quartier();

        let mut boss_nom = boss.alias.clone();

        combat = Combat {
            force_hero: jeu.hero.force,
            force_ennemi: boss.force,
            vie_hero: jeu.hero.vie,
            vie_ennemi: 100,
        };
    
            println!("\nTu te retrouves face au {} !", boss_nom);
    
        loop {
            println!("\nQue veux-tu faire ?");
            println!("1. Attaquer");
            println!("2. Utiliser un objet");
    
            print!("> ");
            io::stdout().flush().unwrap();
            let mut choix = String::new();
            io::stdin().read_line(&mut choix).unwrap();
    
            match choix.trim() {
                "1" => {
                    let (vie_hero, vie_ennemi) = combat.lancer();
                    combat.vie_hero = vie_hero;
                    combat.vie_ennemi = vie_ennemi;
                    jeu.hero.vie = vie_hero;
    
                    if vie_ennemi <= 0 {
                        println!("Tu as vaincu {}!", boss_nom);

                        boss_nom = boss.name.clone();
                        println!("Ou devrait-on dire {}.", boss_nom);

                        println!("{}",jeu.lore.fin);
                        println!("{}",jeu.lore.fin_good);
                        break;
                    } else if vie_hero <= 0 {
                        println!("Tu as perdu...");
                        println!("{}",jeu.lore.fin);
                        println!("{}",jeu.lore.fin_bad);
                        break;
                    }
                },
                "2" => {
                    utilisation_objet::utilisation_objet(jeu);
                },
                _ => println!("Choix invalide."),
            }
        }
    
        println!("{}",jeu.lore.end);
        // Sauvegarde finale
        save::enregistrer_hero(&jeu.hero);
        std::process::exit(0);
    }

    pub fn combat(jeu: &mut Jeu) {
        let quartier = jeu.quartiers.iter_mut()
            .find(|q| q.color == jeu.quartier_actuel)
            .expect("Quartier introuvable");

        let has_gardes = quartier.guards.as_ref().map_or(false, |g| !g.is_empty());
        let has_boss = quartier.boss;

        match (has_gardes, has_boss) {
            (true, _) => {
                // Garde encore présent
                Combat::lancer_combat(jeu);
            }
            (false, true) => {
                // Plus de gardes, mais boss encore en vie
                println!("\nTu arrives devant le boss de ce quartier...");
                Combat::lancer_combat_boss(jeu);
            }
            (false, false) => {
                //Rien à combattre
                println!("Il n'y a plus d'ennemis à combattre ici.");
            }
        }
    }
}
