mod classes;
mod utils;

use crate::utils::ini::initialiser_jeu;
use crate::utils::affichage::Affichage;
use crate::utils::{deplacement, marchandage, discussion};
use crate::classes::jeu::Jeu;
use crate::classes::personnage::Hero;
use crate::classes::action::Action;


use std::io::{self, Write};
use anyhow::Result;


pub struct EtatCombat;

fn main() -> Result<()> {
    


    let mut jeu = initialiser_jeu();
    println!("Jeu initialisé");
    boucle_jeu(&mut jeu)?;
    Ok(())
}


fn boucle_jeu(jeu: &mut Jeu) -> Result<()> {
    let hero = &mut jeu.hero; // Référence au héros du jeu

    loop {
        Affichage::afficher_actions();

        print!("> Choisis une action : ");
        io::stdout().flush().unwrap();
        let mut choix = String::new();
        io::stdin().read_line(&mut choix).unwrap();

        let action = match choix.trim() {
            "1" => Action::Deplacement,
            "2" => Action::UtiliserObjet,
            "3" => Action::Parler,
            "4" => Action::Marchander,
            "5" => Action::Hacker,
            "6" => Action::Combattre,
            "7" => {
                println!("💾 Sauvegarde en cours...");
                continue; // On continue sans effectuer d'action
            }
            "8" => {
                println!("👋 Fin de partie !");
                break;
            }
            _ => {
                println!("⛔ Action inconnue.");
                continue;
            }
        };

        // Gérer l'action en fonction du choix
        match action {
            Action::Deplacement => deplacement::deplacement(jeu),
            Action::UtiliserObjet => utilisation_objet::utilisation_objet(hero),
            Action::Parler => discussion::discussion(jeu),
            Action::Marchander => marchandage::marchandage(jeu),
            Action::Hacker => Affichage::afficher_inventaire(&hero.inventaire),
            Action::Combattre => {
                println!("❤️ Statut : ");
                Affichage::afficher_hero(hero);
            }
            Action::Quitter => break,
        }
    }

    Ok(())
}
