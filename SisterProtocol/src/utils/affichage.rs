use crate::classes::jeu::Jeu;
use crate::classes::quartier::Quartier;
use crate::classes::personnage::Hero;
use crate::classes::inventaire::{Objet, Inventaire, TypeObjet};

pub struct Affichage;

impl Affichage {
    pub fn afficher_message(message: &str) {
        println!("{}", message);
        // ajouter couleur en fonction de la couleur du quartier
    }
    
    pub fn afficher_jeu(jeu: &Jeu) {
        println!("\n=== État du Jeu ===");
        for quartier in &jeu.quartiers {
            println!("- Quartier: {}", quartier.color);
        }
        println!("- Héro: {}", jeu.hero.nom);
    }
    
    pub fn afficher_quartier(quartier: &Quartier) {
        println!("\n=== Quartier: {} ===", quartier.color);
        println!("Gardes: {}", quartier.enemies.len());
        match &quartier.pc {
            Some(ordinateurs) => println!("Ordinateurs: {}", ordinateurs.len()),
            None => println!("Ordinateurs: 0"),
        }
        if quartier.server.is_some() {
            println!("Serveur: Présent");
        }
        if quartier.boss.is_some() {
            println!("Boss: Présent");
        }
        println!("Marchands: Présent");
        println!("PNJs: {}", quartier.residents.len());
        println!("Unlocked: {}", quartier.unlocked);
    }
    
    pub fn afficher_hero(hero: &Hero) {
        println!("\n=== Héro ===");
        println!("Nom: {}", hero.nom);
        println!("Vie: {}", hero.vie);
        println!("Force: {}", hero.force);
        println!("Intelligence: {}", hero.intelligence);
        println!("Argent: {}", hero.argent);
        println!("Position: {}", hero.position);
        println!("Inventaire: {} objets", hero.inventaire.objets.len());
    }

    // Fonction pour afficher les objets de l'inventaire
    pub fn afficher_inventaire(inventaire: &Inventaire) {
        println!("[Inventaire]");
        for objet in &inventaire.objets {
            match objet.type_objet {
                TypeObjet::Nourriture => {
                    println!("Nom: {}, Effet: {}, Type: Nourriture", objet.nom, objet.effet);
                }
                TypeObjet::Amelioration => {
                    println!("Nom: {}, Effet: {}, Type: Amélioration", objet.nom, objet.effet);
                }
                TypeObjet::CarteAcces => {
                    println!("Nom: {}, Effet: {}, Type: Carte d'accès", objet.nom, objet.effet);
                }
            }
        }
    }
}
