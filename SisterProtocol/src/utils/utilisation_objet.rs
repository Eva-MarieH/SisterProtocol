use std::io::{self, Write};

use crate::classes::jeu::Jeu;
use crate::classes::personnage::Hero;
use crate::classes::inventaire::{TypeObjet, Objet, ObjetQuantifie};
use crate::utils::ini::charger_objets;
use crate::utils::save;

pub fn utilisation_objet(jeu: &mut Jeu) {
    let objets_disponibles = match charger_objets() {
        Ok(objets) => objets,
        Err(_) => {
            println!("Impossible de charger les objets.");
            return;
        }
    };

    println!("Objets utilisables dans l'inventaire :");

    loop {
        // Filtre les objets utilisables (en s'assurant qu'il y a des objets avec une quantité > 0)
        let objets_utilisables = jeu.hero.inventory.iter()
            .enumerate()
            .filter_map(|(i, obj_quantifie)| {
                if obj_quantifie.quantity == 0 {
                    return None;
                }
                objets_disponibles.iter().find(|o| o.id == obj_quantifie.id).and_then(|objet| {
                    match objet.type_objet {
                        TypeObjet::Nourriture | TypeObjet::Amelioration => Some((i, obj_quantifie.id, objet)),
                        _ => None,
                    }
                })
            })
            .collect::<Vec<(usize, u8, &Objet)>>();

        // Si l'inventaire est vide
        if objets_utilisables.is_empty() {
            println!("\nInventaire vide.");
            save::enregistrer_hero(&jeu.hero);
            return;
        }

        // Si des objets sont disponibles
        println!("\nVoici les objets disponibles :");
        for (index, (_i, id, objet)) in objets_utilisables.iter().enumerate() {
            let quantite = jeu.hero.inventory.iter().find(|o| o.id == *id).map_or(0, |o| o.quantity);
            println!(
                "{}: {} x{} [{}]",
                index + 1,
                objet.nom,
                quantite,
                match objet.type_objet {
                    TypeObjet::Nourriture => "Nourriture",
                    TypeObjet::Amelioration => "Amélioration",
                    _ => "",
                }
            );
        }

        // L'option pour quitter
        println!("0: Retour");

        print!("> Choisis un objet à utiliser (numéro) ou 0 pour quitter : ");
        io::stdout().flush().unwrap();
        let mut choix = String::new();
        io::stdin().read_line(&mut choix).unwrap();

        let index = match choix.trim().parse::<usize>() {
            Ok(num) if num == 0 => {
                save::enregistrer_hero(&jeu.hero);
                return; // Quitte si l'utilisateur choisit 0
            }
            Ok(num) if num >= 1 && num <= objets_utilisables.len() => num - 1,
            _ => {
                println!("Choix invalide.");
                continue;
            }
        };

        let (_i, _id, objet) = objets_utilisables[index];

        utiliser_objet(&mut jeu.hero, objet);
    }
}

fn utiliser_objet(hero: &mut Hero, objet: &Objet) {
    match objet.type_objet {
        TypeObjet::Nourriture => {
            let vie_avant = hero.vie;
            hero.vie += objet.effet;
            if hero.vie > 100 {
                hero.vie = 100;
            }
            println!(
                "{} utilisé. Vie : {} → {}",
                objet.nom, vie_avant, hero.vie
            );
            retirer_objet(&mut hero.inventory, objet.id);
        }
        TypeObjet::Amelioration => {
            if let Some(am) = &hero.amelioration {
                hero.force -= am.effet;
                println!("Remplacement de {} (effet -{})", am.nom, am.effet);
                ajouter_objet(&mut hero.inventory, am.id);
            }
            println!(
                "{} équipé. Force : {} → {}",
                objet.nom,
                hero.force,
                hero.force + objet.effet
            );
            hero.force += objet.effet;
            hero.amelioration = Some(objet.clone());
            retirer_objet(&mut hero.inventory, objet.id);
        }
        _ => {}
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

fn retirer_objet(inventaire: &mut Vec<ObjetQuantifie>, objet_id: u8) {
    for i in 0..inventaire.len() {
        if inventaire[i].id == objet_id {
            if inventaire[i].quantity > 1 {
                inventaire[i].quantity -= 1; // décrémente si quantité > 1
            } else {
                inventaire.remove(i); // supprime complètement l’objet si quantité == 1
            }
            break; // on sort de la boucle après avoir traité l'objet
        }
    }
}
