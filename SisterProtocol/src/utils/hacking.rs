use crate::utils::minijeux::{mastermind_binaire_random, mastermind_couleur_random, pendu_random};
use crate::utils::affichage::Affichage;
use crate::utils::ini;

use crate::classes::entites::EnnemiHackable;
use crate::classes::jeu::Jeu;
use crate::classes::quartier::Quartier;

use rand::{Rng,rng};

pub fn hacking(jeu: &mut Jeu) {
    let quartier_actuel = jeu.quartiers.iter_mut()
        .find(|q| q.color == jeu.quartier_actuel)
        .expect("Quartier actuel introuvable");

    match prochain_ennemi_hackable(quartier_actuel) {
        Some(EnnemiHackable::Ordinateur(ordi)) => {
            println!("💻 Terminal de sécurité #{} : \"{}\"", ordi.id, ordi.name);
            let jeu_random = rng().random_range(0..=1);
            let result = if jeu_random == 0 {
                mastermind_binaire_random()
            } else {
                mastermind_couleur_random()
            };
            Affichage::afficher_resultat_hacking(&result, &ordi.name);
            if result.is_ok() {
                supprimer_ennemi_hackable(quartier_actuel, &EnnemiHackable::Ordinateur(ordi));
            }
        }

        Some(EnnemiHackable::Serveur(serv)) => {
            println!("🧠 Serveur Central #{} : \"{}\"", serv.id, serv.name);
            let jeu_random = rng().random_range(0..=1);
            let result1 = if jeu_random == 0 {
                mastermind_binaire_random()
            } else {
                mastermind_couleur_random()
            };
            Affichage::afficher_resultat_hacking(&result1, &serv.name);

            if result1.is_ok() {
                println!("🔐 Authentification partielle réussie... lancement du système de sécurité !");
                let result2 = pendu_random();
                Affichage::afficher_resultat_hacking(&result2, &serv.name);
                if result2.is_ok() {
                    println!("🎉 TU AS VAINCU LE SERVEUR!");
                    println!("🚪 Prochain quartier dévérouillé...");
                    supprimer_ennemi_hackable(quartier_actuel, &EnnemiHackable::Serveur(serv));
                }
            }
        }

        None => println!("✅ SYSTEME DE SECURITE HS"),
    }
}

fn supprimer_ennemi_hackable(quartier_actuel: &mut Quartier, ennemi: &EnnemiHackable) {
    match ennemi {
        EnnemiHackable::Ordinateur(ordi) => {
            if let Some(ordinateurs) = &mut quartier_actuel.ordinateurs {
                if let Some(pos) = ordinateurs.iter().position(|&id| id == ordi.id) {
                    ordinateurs.remove(pos);
                }
                if ordinateurs.is_empty() {
                    quartier_actuel.ordinateurs = None;
                }
            }
        }
        EnnemiHackable::Serveur(_) => {
            quartier_actuel.server = None;
        }
    }
}

pub fn prochain_ennemi_hackable(quartier_actuel: &Quartier) -> Option<EnnemiHackable> {
    // Charger les ordinateurs du quartier
    if let Some(ordinateur) = ini::charger_premier_ordinateur_quartier(quartier_actuel){
        return Some(EnnemiHackable::Ordinateur(ordinateur))
    }

    // Charger le serveur du quartier
    if let Some(_) = quartier_actuel.server {
        if let Some(serv) = ini::charger_serveur_quartier(quartier_actuel) {
            return Some(EnnemiHackable::Serveur(serv))
        }
    }

    None
}