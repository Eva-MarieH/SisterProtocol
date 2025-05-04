use std::io::{self, Write};

use crate::classes::jeu::Jeu;
use crate::classes::inventaire::{TypeObjet, Objet, ObjetQuantifie};
use crate::utils::ini::charger_objets;

pub fn utilisation_objet(jeu: &mut Jeu) {
    let objets_disponibles = match charger_objets() {
        Ok(objets) => objets,
        Err(_) => {
            println!("❌ Impossible de charger les objets.");
            return;
        }
    };

    let mut objets_utilisables: Vec<(usize, &ObjetQuantifie, &Objet)> = vec![];

    println!("📦 Objets utilisables dans l'inventaire :");

    for (i, obj_quantifie) in jeu.hero.inventory.iter().enumerate() {
        if let Some(objet_info) = objets_disponibles.iter().find(|o| o.id == obj_quantifie.id) {
            match objet_info.type_objet {
                TypeObjet::Nourriture | TypeObjet::Amelioration => {
                    println!(
                        "{}. {} x{} [{}]",
                        objets_utilisables.len() + 1,
                        objet_info.nom,
                        obj_quantifie.quantity,
                        match objet_info.type_objet {
                            TypeObjet::Nourriture => "Nourriture",
                            TypeObjet::Amelioration => "Amélioration",
                            _ => "",
                        }
                    );
                    objets_utilisables.push((i, obj_quantifie, objet_info));
                }
                _ => {}
            }
        }
    }

    if objets_utilisables.is_empty() {
        println!("ℹ️ Aucun objet utilisable.");
        return;
    }

    print!("> Choisis un objet à utiliser : ");
    io::stdout().flush().unwrap();
    let mut choix = String::new();
    io::stdin().read_line(&mut choix).unwrap();

    let index = match choix.trim().parse::<usize>() {
        Ok(num) if num >= 1 && num <= objets_utilisables.len() => num - 1,
        _ => {
            println!("⛔ Choix invalide.");
            return;
        }
    };

    let (inv_index, _, objet) = objets_utilisables[index];

    match objet.type_objet {
        TypeObjet::Nourriture => {
            let vie_avant = jeu.hero.vie;
            jeu.hero.vie += objet.effet;
            if jeu.hero.vie > 100 {
                jeu.hero.vie = 100;
            }
            println!(
                "🍽️ {} utilisé. Vie : {} → {}",
                objet.nom, vie_avant, jeu.hero.vie
            );
            retirer_objet(&mut jeu.hero.inventory, objet.id);
        }
        TypeObjet::Amelioration => {
            if let Some(am) = &jeu.hero.amelioration {
                jeu.hero.force -= am.effet;
                println!(
                    "♻️ Remplacement de {} (effet -{})",
                    am.nom, am.effet
                );
                // Remettre l'ancienne amélioration dans l'inventaire
                ajouter_objet(&mut jeu.hero.inventory, am.id);
            }
            println!(
                "🛠️ {} équipé. Force : {} → {}",
                objet.nom,
                jeu.hero.force,
                jeu.hero.force + objet.effet
            );
            jeu.hero.force += objet.effet;
            jeu.hero.amelioration = Some(objet.clone());
            retirer_objet(&mut jeu.hero.inventory, objet.id);
        }
        _ => {}
    }
}

fn retirer_objet(inventaire: &mut Vec<ObjetQuantifie>, objet_id: u8) {
    if let Some(pos) = inventaire.iter().position(|o| o.id == objet_id) {
        if inventaire[pos].quantity > 1 {
            inventaire[pos].quantity -= 1;
        } else {
            inventaire.remove(pos);
        }
    }
}

fn ajouter_objet(inventaire: &mut Vec<ObjetQuantifie>, objet_id: u8) {
    if let Some(pos) = inventaire.iter().position(|o| o.id == objet_id) {
        inventaire[pos].quantity += 1;
    } else {
        inventaire.push(ObjetQuantifie {
            id: objet_id,
            quantity: 1,
        });
    }
}
