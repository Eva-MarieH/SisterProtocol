use std::{fs, io::{self, Write}};
use serde_json::Value;
use anyhow::{Result, Context};
use rand::Rng;

use crate::classes::quartier::Quartier;
use crate::classes::personnage::{Resident, Hero};

// Charger le quartier en fonction de la position du héros
pub fn charger_quartier(hero: &Hero) -> Result<Option<Quartier>> {
    // Lire le fichier districts.json
    let contenu = fs::read_to_string("assets/districts.json")
        .context("Erreur de lecture de districts.json")?;

    // Désérialiser en un Vec<QuartierBrut>
    let quartiers: Vec<Quartier> = serde_json::from_str(&contenu)
        .context("Erreur de parsing de districts.json")?;

    // Rechercher le quartier correspondant à la couleur du héros
    let quartier = quartiers
        .into_iter()
        .find(|q| q.color == hero.position);

    Ok(quartier)
}

// Charger les résidents du quartier
pub fn charger_residents_quartier(quartier: &Quartier) -> Result<Vec<Resident>> {
    // Lire les résidents depuis le fichier JSON
    let contenu_residents = fs::read_to_string("assets/residents.json")
        .context("Erreur de lecture de residents.json")?;
    let residents: Vec<Resident> = serde_json::from_str(&contenu_residents)
        .context("Erreur de parsing de residents.json")?;

    // Récupérer les résidents du quartier actuel en fonction des indices
    let citoyens = quartier.residents.iter()
        .filter_map(|&id| residents.get(id).cloned()) // Cloner les résidents pour avoir une copie
        .collect::<Vec<Resident>>();

    Ok(citoyens)
}

// Afficher les dialogues d'un résident en fonction du contexte
fn afficher_dialogue(resident: &Resident, context: &str) {
    // Chercher un dialogue spécifique au contexte
    if let Some(dialogue) = resident.dialogues.iter().find(|d| d.context == context) {
        println!("{}", dialogue.text); // Afficher le texte du dialogue
    } else {
        println!("Aucun dialogue pour ce contexte.");
    }
}

// Fonction pour discuter avec un résident
pub fn discuter_avec_resident(residents: &[Resident]) {
    if residents.is_empty() {
        println!("Il n'y a pas de résident à qui parler dans ce quartier.");
        return;
    }

    println!("Choisis un résident avec qui tu veux parler :");
    
    // Affichage des résidents avec un index pour que le joueur puisse choisir
    for (index, resident) in residents.iter().enumerate() {
        println!("{}: {}", index + 1, resident.name); // Affiche l'index (1-based) et le nom
    }

    // Demander à l'utilisateur de faire un choix
    print!("Entrez le numéro du résident : ");
    io::stdout().flush().unwrap(); // Assurer que le message s'affiche avant d'attendre l'entrée

    let mut choix = String::new();
    io::stdin().read_line(&mut choix).unwrap();
    
    // Convertir l'entrée en nombre et vérifier que c'est un choix valide
    match choix.trim().parse::<usize>() {
        Ok(num) if num >= 1 && num <= residents.len() => {
            // Sélectionner le résident choisi
            let resident_choisi = &residents[num - 1];
            println!("\nTu parles avec {}...", resident_choisi.name);

            // Afficher le premier dialogue
            afficher_dialogue(resident_choisi, "first");

            // Boucle de discussion
            let mut continuer = true;
            while continuer {
                // Demander à l'utilisateur quel contexte il souhaite explorer
                println!("\nChoisis un sujet de discussion :");
                println!("1: La ville");
                println!("2: Les ennemis");
                println!("3: Ta soeur");
                println!("4: Partir");

                print!("Entrez votre choix : ");
                io::stdout().flush().unwrap(); // Assurer que le message s'affiche avant d'attendre l'entrée

                let mut choix_contexte = String::new();
                io::stdin().read_line(&mut choix_contexte).unwrap();
                
                match choix_contexte.trim() {
                    "1" => {
                        afficher_dialogue(resident_choisi, "lore");
                    }
                    "2" => {
                        afficher_dialogue(resident_choisi, "ennemy");
                    }
                    "3" => {
                        afficher_dialogue(resident_choisi, "sister");
                    }
                    "4" => {
                        // Afficher le dernier dialogue (context = "end") et quitter la boucle
                        afficher_dialogue(resident_choisi, "end");
                        continuer = false;
                    }
                    _ => {
                        println!("Choix invalide, essaye encore.");
                    }
                }
            }
        }
        _ => println!("Choix invalide."),
    }
}

// Fonction principale pour démarrer la discussion
pub fn discussion(hero: &Hero) {
    match charger_quartier(hero) {
        Ok(Some(q)) => {
            // Charger les résidents du quartier
            let citoyens = charger_residents_quartier(&q).unwrap();

            // Afficher les citoyens (résidents)
            if citoyens.is_empty() {
                println!("Aucun citoyen dans ce quartier.");
            } else {
                // Permettre au joueur de discuter avec les résidents
                discuter_avec_resident(&citoyens);
            }
        },
        _ => println!("Quartier introuvable."),
    }
}
