mod classes;
mod utils;

use crate::utils::affichage::Affichage;
use utils::minijeux;
use utils::deplacement;
use utils::deplacement;
use std::fs;
use std::io::{self, Write};
use serde_json::Value;
use anyhow::{Result, Context};
use rand::Rng;

pub struct EtatCombat;

fn main() -> Result<()> {
    use classes::personnage::Hero;

    let mut hero = Hero {
        nom: String::from("John Doe"),
        vie: 100,
        force: 10,
        intelligence: 15,
        argent: 50,
        position: String::from("bleu"),
        position: String::from("bleu"),
        inventaire: classes::inventaire::Inventaire { objets: vec![] },
        amelioration: None,
    };

    boucle_jeu(&mut hero)?;
    Ok(())
}

fn afficher_actions() {
    println!("\n🕹️  Actions possibles :");
    println!("1. Se déplacer vers un autre quartier");
    println!("1. Se déplacer vers un autre quartier");
    println!("2. Utiliser un objet de l'inventaire");
    println!("3. Parler à un citoyen");
    println!("4. Marchander");
    println!("5. Hacker");
    println!("6. Combattre");
    println!("7. Quitter le jeu");
    println!("3. Parler à un citoyen");
    println!("4. Marchander");
    println!("5. Hacker");
    println!("6. Combattre");
    println!("7. Quitter le jeu");
}

fn boucle_jeu(hero: &mut classes::personnage::Hero) -> Result<()> {
    loop {
        afficher_actions();

        print!("> Choisis une action : ");
        io::stdout().flush().unwrap();
        let mut choix = String::new();
        io::stdin().read_line(&mut choix).unwrap();

        match choix.trim() {
            "1" =>utils::deplacement::deplacement(hero),
            "2" =>{
                println!("🎒 Quel objet veux-tu utiliser ?");
                println!("{:?}", Affichage::afficher_inventaire(&hero.inventaire));
                print!("> Choisis un objet : ");
                io::stdout().flush().unwrap();
                let mut choix2 = String::new();
                io::stdin().read_line(&mut choix2).unwrap();
                println!("🎒 Tu utilises l'objet {}...", choix2);


            },
            "3" => utils::discussion::discussion(hero),
            "4" => utils::marchandage::marchand(hero),
            "5" => println!("📦 Inventaire : {:?}", Affichage::afficher_inventaire(&hero.inventaire)),
            "6" => {
                println!("❤️ Statut : ");
                Affichage::afficher_hero(&hero);
            }
            "7" => println!("💾 Sauvegarde en cours..."),
            "8" => {
                println!("👋 Fin de partie !");
                break;
            }
            _ => println!("⛔ Action inconnue."),
        }
    }
    Ok(())
}