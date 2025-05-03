use crate::classes;
use crate::utils::minijeux;
use crate::utils::minijeux::{mastermind_binaire_random, mastermind_couleur_random, pendu_random};

use std::fs;
use serde_json::Value;
use rand::Rng;
use anyhow::{Result, Context};

pub fn haking() -> Result<()> {
    match prochain_ennemi()? {
        Some(ref ennemi) if ennemi == "ordinateur" => {
            println!("💻 Tu attaques un terminal de sécurité...");
            let jeu = rand::rng().random_range(0..2);
            let result = if jeu == 0 {
                mastermind_binaire_random()
            } else {
                mastermind_couleur_random()
            };
            afficher_resultat_hacking(&result, "l'ordinateur");
        }
        Some(ref ennemi) if ennemi == "serveur" => {
            println!("🧠 ACCÈS AU SERVEUR CENTRAL EN COURS...");
            let jeu = rand::rng().random_range(0..2);
            let result1 = if jeu == 0 {
                mastermind_binaire_random()
            } else {
                mastermind_couleur_random()
            };

            afficher_resultat_hacking(&result1, "le serveur");

            if result1.is_ok() {
                println!("🔐 Authentification partielle réussie... Lancement du système de sécurité !");
                let result2 = pendu_random();
                if result2.is_ok() {
                    println!("🎉 TU AS VAINCU LE SERVEUR !");
                    supprimer_premier_ennemi()?;
                } else {
                    afficher_resultat_hacking(&result2, "le serveur");
                }
            }
        }
        Some(ref ennemi) => {
            println!("❓ Ennemi inconnu : {}", ennemi);
        }
        None => {
            println!("✅ Tous les ennemis ont été vaincus !");
        }
    }
    Ok(())
}

fn afficher_resultat_hacking(result: &Result<(), anyhow::Error>, cible: &str) {
    match result {
        Ok(_) => println!("✅ Tu as neutralisé {}", cible),
        Err(e) => println!("💀 Échec contre {} : {}", cible, e),
    }
}

fn supprimer_premier_ennemi() -> Result<()> {
    let contenu = fs::read_to_string("assets/etat.json")
        .context("Erreur de lecture du fichier etat.json")?;
    let mut json: Value = serde_json::from_str(&contenu)
        .context("Erreur de parsing du fichier JSON")?;

    for cle in ["garde", "ordinateur", "serveur"] {
        if json.get(cle).is_some() {
            json.as_object_mut().unwrap().remove(cle);
            fs::write("assets/etat.json", serde_json::to_string_pretty(&json)?)?;
            break;
        }
    }
    Ok(())
}

fn prochain_ennemi() -> Result<Option<String>> {
    let contenu = fs::read_to_string("assets/etat.json")
        .context("Impossible de lire le fichier etat.json")?;
    let json: Value = serde_json::from_str(&contenu)?;

    if json.get("garde").is_some() {
        Ok(Some("garde".to_string()))
    } else if json.get("ordinateur").is_some() {
        Ok(Some("ordinateur".to_string()))
    } else if json.get("serveur").is_some() {
        Ok(Some("serveur".to_string()))
    } else {
        Ok(None)
    }
}
