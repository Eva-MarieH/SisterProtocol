mod classes;
mod utils;

use crate::utils::ini::{initialiser_jeu, continue_jeu};
use crate::utils::action;

use anyhow::{Result,Context};
use utils::affichage::Affichage;
use std::io::{self, Write};
pub struct EtatCombat;

fn main() -> Result<()> {
    loop{

        println!("Bienvenue dans SisterProtocol., Que voulez-vous faire ? ");
        print!("1. Nouvelle partie\n2. Charger une partie\n3. Quitter\n> ");
        io::stdout().flush().unwrap();
        let mut choix = String::new();
        io::stdin().read_line(&mut choix).unwrap();

        // Gérer l'action en fonction du choix
        match choix.trim() {
            "1" => {   
                let mut jeu = initialiser_jeu();
                Affichage::afficher_message("Jeu initialisé");
                action::boucle_jeu(&mut jeu);
            },
            "2" => {
                let mut jeu = continue_jeu();
                Affichage::afficher_message("Jeu chargé");
                Affichage::afficher_hero(&jeu.hero);
                let quartier_actuel = jeu.quartiers.iter_mut().find(|quartier| quartier.color == jeu.quartier_actuel)
                 .context("Quartier actuel introuvable").unwrap();
                Affichage::afficher_quartier(quartier_actuel);
                action::boucle_jeu(&mut jeu);
            }
            "3" => {
                break;
            }
            _ => {
                println!("⛔ Action inconnue.");
            }
        }
    }

    Ok(())
}