use crate::classes::jeu::Jeu;
use crate::classes::quartier::Quartier;
use crate::classes::personnage::Hero;

pub struct Affichage;

impl Affichage {
    pub fn afficher_message(message: &str) {
        println!("{}", message);
    }
    
    pub fn afficher_jeu(jeu: &Jeu) {
        println!("\n=== État du Jeu ===");
        for quartier in &jeu.quartiers {
            println!("- Quartier: {}", quartier.couleur);
        }
    }
    
    pub fn afficher_quartier(quartier: &Quartier) {
        println!("\n=== Quartier: {} ===", quartier.couleur);
        println!("Cachettes: {}", quartier.cachettes.len());
        println!("Gardes: {}", quartier.gardes.len());
        println!("Ordinateurs: {}", quartier.ordinateurs.len());
        println!("Serveur: Présent");
        if quartier.boss.is_some() {
            println!("Boss: Présent");
        }
        println!("Marchands: {}", quartier.marchands.len());
        println!("PNJs: {}", quartier.pnjs.len());
    }
    
    pub fn afficher_hero(hero: &Hero) {
        println!("\n=== Héro ===");
        println!("Vie: {}", hero.vie);
        println!("Force: {}", hero.force);
        println!("Intelligence: {}", hero.intelligence);
        println!("Argent: {}", hero.argent);
        println!("Position: {}", hero.position);
        println!("Inventaire: {} objets", hero.inventaire.objets.len());
    }
}