use crate::utils::affichage::Affichage;
use crate::utils::{deplacement, discussion, hacking, marchandage, save, utilisation_objet};
use crate::classes::jeu::Jeu;
use crate::classes::action::Action;
use crate::classes::combat::Combat;

use std::io::{self, Write};

pub fn boucle_jeu(jeu: &mut Jeu) {

    loop {
        Affichage::afficher_actions();

        print!("> Choisis une action : ");
        io::stdout().flush().unwrap();
        let mut choix = String::new();
        io::stdin().read_line(&mut choix).unwrap();
        println!("");

        let action = match choix.trim() {
            "1" => Action::Deplacement,
            "2" => Action::UtiliserObjet,
            "3" => Action::Parler,
            "4" => Action::Marchander,
            "5" => Action::Hacker,
            "6" => Action::Combattre,
            "7" => Action::Quitter,
            _ => {
                println!("⛔ Action inconnue.");
                continue;
            }
        };

        // Gérer l'action en fonction du choix
        match action {
            Action::Deplacement => deplacement::deplacement(jeu),
            Action::UtiliserObjet => utilisation_objet::utilisation_objet(jeu),
            Action::Parler => discussion::discussion(jeu),
            Action::Marchander => marchandage::marchandage(jeu),
            Action::Hacker => {
                if jeu.peut_pirater() {
                    println!("🧠 Tentative de piratage en cours...");
                    hacking::hacking(jeu);
                } else {
                    println!("🚫 Impossible de pirater : des gardes sont encore présents !");
                }
            },
            Action::Combattre => {
                if jeu.peut_combattre() {
                    Combat::lancer_combat(jeu);
                }
                else {
                    println!("🚫 Impossible de combattre : vous n'avez plus de vie !");
                }
            },
            Action::Quitter => {
                save::enregistrer_hero(&jeu.hero);
                save::enregistrer_quartiers(&jeu.quartiers);
                break
            },
        }
    }
}