mod classes;
mod utils;

use crate::utils::ini::{initialiser_jeu, continue_jeu};
use crate::utils::action;
use crate::utils::save;
use crate::utils::affichage::Affichage;

use anyhow::{Result,Context};
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
                Affichage::afficher_message("\n Jeu initialisé");

                println!("\n{}", jeu.lore.debut);

                print!("\nVotre nom est flou dans votre mémoire mais vous vous concentez : ");
                io::stdout().flush().unwrap();

                let mut nom_hero = String::new();
                io::stdin().read_line(&mut nom_hero).unwrap();
                jeu.hero.name = nom_hero.trim().to_string();
                save::enregistrer_hero(&jeu.hero);

                println!("\nVous voilà au quartier {}.", jeu.quartier_actuel);
                print!("{}\n", jeu.lore.bleu);

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
                println!("\nAucun fichier de sauvegarde trouvé.");
            }
            "3" => {
                println!("\nMerci d'avoir joué à SisterProtocol !");
                break;
            }
            _ => {
                println!("\nAction inconnue.");
            }
        }
    }
    Ok(())    
}