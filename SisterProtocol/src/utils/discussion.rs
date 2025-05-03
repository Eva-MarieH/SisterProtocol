use std::{fs, io::{self, Write}};

use crate::classes::personnage::{Resident, Hero};
use crate::utils::affichage::Affichage;
use crate::utils::ini;


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
            Affichage::afficher_dialogue_residents(resident_choisi, "first");

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
                        Affichage::afficher_dialogue_residents(resident_choisi, "lore");
                    }
                    "2" => {
                        Affichage::afficher_dialogue_residents(resident_choisi, "ennemy");
                    }
                    "3" => {
                        Affichage::afficher_dialogue_residents(resident_choisi, "sister");
                    }
                    "4" => {
                        // Afficher le dernier dialogue (context = "end") et quitter la boucle
                        Affichage::afficher_dialogue_residents(resident_choisi, "end");
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
    match ini::charger_quartier(hero) {
        Ok(Some(q)) => {
            // Charger les résidents du quartier
            let citoyens = ini::charger_residents_quartier(&q).unwrap();

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
