mod classes;
mod utils;

use crate::utils::affichage::Affichage;
use utils::minijeux;
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
        position: String::from("Quartier bleu"),
        inventaire: classes::inventaire::Inventaire { objets: vec![] },
        amelioration: None,
    };

    boucle_jeu(&mut hero)?;
    Ok(())
}

fn afficher_actions() {
    println!("\n🕹️  Actions possibles :");
    println!("1. Aller à un autre lieu");
    println!("2. Utiliser un objet de l'inventaire");
    println!("3. Parler à un PNJ");
    println!("4. Combattre/hacker");
    println!("5. Afficher l'inventaire");
    println!("6. Afficher le statut du joueur");
    println!("7. Sauvegarder la partie");
    println!("8. Quitter le jeu");
}

fn boucle_jeu(hero: &mut classes::personnage::Hero) -> Result<()> {
    loop {
        afficher_actions();

        print!("> Choisis une action : ");
        io::stdout().flush().unwrap();
        let mut choix = String::new();
        io::stdin().read_line(&mut choix).unwrap();

        match choix.trim() {
            "1" => println!("🚶 Tu te déplaces vers un autre lieu..."),
            "2" => println!("🎒 Quel objet veux-tu utiliser ?"),
            "3" => println!("🗣️ Tu engages la discussion avec un personnage..."),
            "4" => {
                println!("⚔️ Tu rencontres un ennemi...");
                combat()?;
            }
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

fn supprimer_premier_ennemi() -> Result<()> {
    let contenu = fs::read_to_string("assets/etat.json")?;
    let mut json: Value = serde_json::from_str(&contenu)?;

    for cle in ["garde", "ordinateur", "serveur"] {
        if json.get(cle).is_some() {
            json.as_object_mut().unwrap().remove(cle);
            fs::write("assets/etat.json", serde_json::to_string_pretty(&json)?)?;
            break;
        }
    }

    Ok(())
}

fn afficher_resultat_combat(result: Result<(), anyhow::Error>, cible: &str) {
    match result {
        Ok(_) => println!("✅ Tu as neutralisé {}", cible),
        Err(e) => println!("💀 Échec contre {} : {}", cible, e),
    }
}

fn combat() -> Result<()> {
    match prochain_ennemi()? {
        Some(ref ennemi) if ennemi == "garde" => {
            println!("🛡️ Un garde t'intercepte !");
            let result = utils::combat::combat(); // Assurez-vous que `utils::combat` existe
            if result.is_ok() {
                supprimer_premier_ennemi()?;
            }
        }
        Some(ref ennemi) if ennemi == "ordinateur" => {
            println!("💻 Tu attaques un terminal de sécurité...");
            let jeu = rand::rng().random_range(0..2);
            let result = if jeu == 0 {
                minijeux::mastermind_binaire_random()
            } else {
                minijeux::mastermind_couleur_random()
            };
            afficher_resultat_combat(result, "l'ordinateur");
        }
        Some(ref ennemi) if ennemi == "serveur" => {
            println!("🧠 ACCÈS AU SERVEUR CENTRAL EN COURS...");
            let jeu = rand::rng().random_range(0..2);
            let result1 = if jeu == 0 {
                minijeux::mastermind_binaire_random()
            } else {
                minijeux::mastermind_couleur_random()
            };

            if result1.is_err() {
                afficher_resultat_combat(result1, "le serveur");
                return Ok(());
            }

            println!("🔐 Authentification partielle réussie... Lancement du système de sécurité !");
            let result2 = minijeux::pendu_random();
            if result2.is_ok() {
                println!("🎉 TU AS VAINCU LE SERVEUR !");
                supprimer_premier_ennemi()?;
            } else {
                afficher_resultat_combat(result2, "le serveur");
            }
        }
        Some(ref ennemi) => {
            println!("Unknown enemy: {}", ennemi);
        }
        None => {
            println!("✅ Tous les ennemis ont été vaincus !");
        }
    }
    Ok(())
}
