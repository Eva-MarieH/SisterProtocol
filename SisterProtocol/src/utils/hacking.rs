use crate::utils::minijeux::{mastermind_binaire_random, mastermind_couleur_random, pendu_random};
use crate::classes::entites::EnnemiHackable;
use crate::utils::affichage::Affichage;
use crate::classes::jeu::Jeu;
use crate::classes::quartier::Quartier;

use rand::Rng;
use anyhow::{Result, Context};

pub fn haking(jeu: &mut Jeu) -> Result<()> {
    let quartier_actuel = jeu.quartiers.iter_mut().find(|quartier| quartier.color == jeu.quartier_actuel)
        .context("Quartier actuel introuvable")?;
    
    match prochain_ennemi_hackable(quartier_actuel)? {
        Some(ennemi) => {
            match ennemi {
                EnnemiHackable::Ordinateur { id, name } => {
                    println!("💻 Tu attaques le terminal de sécurité #{id} : \"{name}\"");
                    let jeu_random = rand::rng().random_range(0..2); // Utilisation de random_range
                    let result = if jeu_random == 0 {
                        mastermind_binaire_random()
                    } else {
                        mastermind_couleur_random()
                    };
                    Affichage::afficher_resultat_hacking(&result, &name);
                    if result.is_ok() {
                        supprimer_ennemi_hackable(quartier_actuel, &EnnemiHackable::Ordinateur { id, name })?;
                    }
                }
                EnnemiHackable::Serveur { id, name } => {
                    println!("🧠 ACCÈS AU SERVEUR CENTRAL #{id} : \"{name}\" EN COURS...");
                    let jeu_random = rand::rng().random_range(0..2); // Utilisation de random_range
                    let result1 = if jeu_random == 0 {
                        mastermind_binaire_random()
                    } else {
                        mastermind_couleur_random()
                    };

                    Affichage::afficher_resultat_hacking(&result1, &name);

                    if result1.is_ok() {
                        println!("🔐 Authentification partielle réussie... Lancement du système de sécurité !");
                        let result2 = pendu_random();
                        if result2.is_ok() {
                            println!("🎉 TU AS VAINCU LE SERVEUR !");
                            supprimer_ennemi_hackable(quartier_actuel, &EnnemiHackable::Serveur { id, name })?;
                        } else {
                            Affichage::afficher_resultat_hacking(&result2, &name);
                        }
                    }
                }
            }
        }
        None => println!("✅ Tous les ennemis ont été vaincus !"),
    }

    Ok(())
}

fn supprimer_ennemi_hackable(quartier_actuel: &mut Quartier, ennemi: &EnnemiHackable) -> Result<()> {
    match ennemi {
        EnnemiHackable::Ordinateur { id, .. } => {
            if let Some(ordinateurs) = &mut quartier_actuel.ordinateurs {
                if let Some(pos) = ordinateurs.iter().position(|&x| x == *id) {
                    ordinateurs.remove(pos);
                    // Si la liste des ordinateurs devient vide, on met la valeur à None
                    if ordinateurs.is_empty() {
                        quartier_actuel.ordinateurs = None;
                    }
                }
            }
        }
        EnnemiHackable::Serveur { id, .. } => {
            if quartier_actuel.server == Some(*id) {
                quartier_actuel.server = None;
            }
        }
    }
    
    Ok(())
}

pub fn prochain_ennemi_hackable(quartier_actuel: &Quartier) -> Result<Option<EnnemiHackable>> {
    if let Some(ordinateurs) = &quartier_actuel.ordinateurs {
        if let Some(&id) = ordinateurs.get(0) {
            // Retourne le premier ordinateur de la liste comme ennemi hackable
            return Ok(Some(EnnemiHackable::Ordinateur { id, name: format!("Ordinateur #{id}") }));
        }
    }

    if let Some(server_id) = quartier_actuel.server {
        // Si un serveur existe, le retourne comme ennemi hackable
        return Ok(Some(EnnemiHackable::Serveur { id: server_id, name: "Serveur Central".to_string() }));
    }

    // Si aucun ennemi hackable n'est trouvé, retourne None
    Ok(None)
}