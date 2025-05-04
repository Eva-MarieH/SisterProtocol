mod classes;
mod utils;

use crate::utils::ini::{initialiser_jeu, continue_jeu};
use crate::utils::action;

use anyhow::{Result,Context};
use utils::affichage::Affichage;
use std::io::{self, Write};
use std::path::Path;
pub struct EtatCombat;

fn main() -> Result<()> {
    println!("\nBienvenue dans SisterProtocol.");
    loop {
        let save_existe = Path::new("assets/saves").exists();
        
        println!("\nQue voulez-vous faire ?");
        println!("1. Nouvelle partie");
        println!("2. Charger une partie");
        println!("3. Quitter");
        print!("> ");
        io::stdout().flush().unwrap();
    
        let mut choix = String::new();
        io::stdin().read_line(&mut choix).unwrap();
    
        match choix.trim() {
            "1" => {
                let mut jeu = initialiser_jeu();
                Affichage::afficher_message("Jeu initialisé");
                action::boucle_jeu(&mut jeu);
            }
            "2" if save_existe => {
                let mut jeu = continue_jeu();
                Affichage::afficher_message("Jeu chargé");
                Affichage::afficher_hero(&jeu.hero);
                let quartier_actuel = jeu.quartiers.iter_mut()
                    .find(|quartier| quartier.color == jeu.quartier_actuel)
                    .context("Quartier actuel introuvable")?;
                Affichage::afficher_quartier(quartier_actuel);
                action::boucle_jeu(&mut jeu);
            }
            "2" => {
                println!("\n⛔ Aucun fichier de sauvegarde trouvé.");
            }
            "3" => {
                println!("\n👋 Merci d'avoir joué à SisterProtocol !");
                break;
            }
            _ => {
                println!("\n⛔ Action inconnue.");
            }
        }
    }
    Ok(())    
}