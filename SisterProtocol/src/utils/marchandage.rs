use crate::classes::personnage::{Hero, Marchand}; 
use crate::classes::inventaire::{Objet, ObjetQuantifie, TypeObjet};
use crate::classes::jeu::Jeu;
use crate::utils::affichage::Affichage;
use crate::utils::{ini, save}; 


use std::io::{self, Write};

fn ajouter_a_inventaire(inventaire: &mut Vec<ObjetQuantifie>, id: u8, quantite: u32) {
    if let Some(obj) = inventaire.iter_mut().find(|o| o.id == id) {
        obj.quantity += quantite;
    } else {
        inventaire.push(ObjetQuantifie { id, quantity: quantite });
    }
}


fn retirer_du_inventaire(inventaire: &mut Vec<ObjetQuantifie>, id: u8, quantite: u32) -> bool {
    if let Some(pos) = inventaire.iter().position(|o| o.id == id && o.quantity >= quantite) {
        let obj = &mut inventaire[pos];
        obj.quantity -= quantite;
        if obj.quantity == 0 {
            inventaire.remove(pos);
        }
        true
    } else {
        false
    }
}
fn vendre_objet(hero: &mut Hero, marchand: &mut Marchand) {
    loop{
         // Liste des objets que le héros peut vendre
        let objets_disponibles = match ini::charger_objets() {
            Ok(objets) => objets,
            Err(_) => {
                println!("Impossible de charger les objets.");
                return;
            }
        };

        let mut objets_vendables: Vec<(usize, &ObjetQuantifie, &Objet)> = vec![];

        // Afficher les objets vendables (Nourriture et Amélioration)
        println!("\n--- Tes objets à vendre ---");

        // Ajouter les objets utilisables à la liste pour pouvoir les manipuler après
        for (i, obj_quantifie) in hero.inventory.iter().enumerate() {
            if let Some(objet_info) = objets_disponibles.iter().find(|o| o.id == obj_quantifie.id) {
                match objet_info.type_objet {
                    TypeObjet::Nourriture | TypeObjet::Amelioration => {
                        objets_vendables.push((i, obj_quantifie, objet_info));
                    }
                    _ => {}
                }
            }
        }

        if objets_vendables.is_empty() {
            println!("ℹ️ Aucun objet utilisable.");
            return;
        }

        // Afficher les objets avec des numéros
        for (index, (_i, obj_quantifie, objet)) in objets_vendables.iter().enumerate() {
            println!(
                "{}: {} x{} [{}]",
                index + 1,  // Ajoute un numéro à chaque objet
                objet.nom,
                obj_quantifie.quantity,
                match objet.type_objet {
                    TypeObjet::Nourriture => "Nourriture",
                    TypeObjet::Amelioration => "Amélioration",
                    _ => "",
                }
            );
        }

        println!("0: Retour");

        println!("Argent du marchand : {} cr", marchand.money);
        print!("> Choisis un objet à utiliser (numéro) : ");
        io::stdout().flush().unwrap();
        let mut choix = String::new();
        io::stdin().read_line(&mut choix).unwrap();

        // Vérifie si l'index du choix est valide
        let index = match choix.trim().parse::<usize>() {
            Ok(num) if num == 0 => {
                return; // Quitte si l'utilisateur choisit 0
            }
            Ok(num) if num >= 1 && num <= objets_vendables.len() => num - 1,
            _ => {
                println!("Choix invalide.");
                return;
            }
        };

        // Récupère l'objet choisi
        let (_, _, objet) = objets_vendables[index];

        let prix = objet.prix.unwrap_or(0);
        if retirer_du_inventaire(&mut hero.inventory, objet.id, 1) {
            if marchand.money >= prix {
                hero.argent += prix;
                marchand.money -= prix;
                ajouter_a_inventaire(&mut marchand.inventory, objet.id, 1);
                println!("Vendu pour {} crédits.", prix);
            } else {
                println!("Le marchand n'a pas assez d'argent.");
                ajouter_a_inventaire(&mut hero.inventory, objet.id, 1); // remboursement
            }
        }

    }
   
}

fn acheter_objet(hero: &mut Hero, marchand: &mut Marchand) {
    loop {
        // Liste des objets que le marchand peut vendre
        let objets_disponibles = match ini::charger_objets() {
            Ok(objets) => objets,
            Err(_) => {
                println!("Impossible de charger les objets.");
                return;
            }
        };

        let mut objets_achetables: Vec<(usize, &ObjetQuantifie, &Objet)> = vec![];

        // Afficher les objets vendables (Nourriture et Amélioration)
        println!("\n--- Objets disponibles à l'achat ---");

        // Ajouter les objets utilisables à la liste pour pouvoir les manipuler après
        for (i, obj_quantifie) in marchand.inventory.iter().enumerate() {
            if let Some(objet_info) = objets_disponibles.iter().find(|o| o.id == obj_quantifie.id) {
                match objet_info.type_objet {
                    TypeObjet::Nourriture | TypeObjet::Amelioration => {
                        objets_achetables.push((i, obj_quantifie, objet_info));
                    }
                    _ => {}
                }
            }
        }

        if objets_achetables.is_empty() {
            println!("Aucun objet à vendre.");
            return;
        }

        // Afficher les objets avec des numéros
        for (index, (_i, obj_quantifie, objet)) in objets_achetables.iter().enumerate() {
            println!(
                "{}: {} x{} [{}] -> {} crédits",
                index + 1,  // Ajoute un numéro à chaque objet
                objet.nom,
                obj_quantifie.quantity,
                match objet.type_objet {
                    TypeObjet::Nourriture => "Nourriture",
                    TypeObjet::Amelioration => "Amélioration",
                    _ => "",
                },
                objet.prix.unwrap_or(0)
            );
        }

        println!("0: Retour");

        println!("\n--- Votre argent : {} ---", hero.argent);
        print!("> Choisis un objet à acheter (numéro) : ");
        io::stdout().flush().unwrap();
        let mut choix = String::new();
        io::stdin().read_line(&mut choix).unwrap();

        // Vérifie si l'index du choix est valide
        let index = match choix.trim().parse::<usize>() {
            Ok(num) if num == 0 => {
                return; // Quitte si l'utilisateur choisit 0
            }
            Ok(num) if num >= 1 && num <= objets_achetables.len() => num - 1,
            _ => {
                println!("Choix invalide.");
                continue; // Repart à la boucle si le choix est invalide
            }
        };

        // Récupère l'objet choisi
        let (_, _, objet) = objets_achetables[index];
    
        let prix = objet.prix.unwrap_or(0);
        if hero.argent < prix {
            println!("Tu n'as pas assez d'argent.");
            continue; // Repart à la boucle si l'utilisateur n'a pas assez d'argent
        }

        // Effectuer l'achat si l'objet est disponible
        if retirer_du_inventaire(&mut marchand.inventory, objet.id, 1) {
            hero.argent -= prix;
            marchand.money += prix;
            ajouter_a_inventaire(&mut hero.inventory, objet.id, 1);
            println!("Achat réussi !");
        } else {
            println!("Objet plus disponible.");
            continue; // Repart à la boucle si l'objet n'est plus disponible
        }
    }
}


pub fn discuter_avec_marchand(hero: &mut Hero, marchand: &mut Marchand) {
    println!("Vous parlez avec {}.\n", marchand.name);
    Affichage::afficher_dialogue_marchand(marchand, "start");

    loop {
        println!("\nQue voulez-vous faire ?");
        println!("1. Vendre");
        println!("2. Acheter");
        println!("3. Partir");

        print!("> ");
        io::stdout().flush().unwrap();
        let mut choix = String::new();
        io::stdin().read_line(&mut choix).unwrap();

        match choix.trim() {
            "1" => {
                vendre_objet(hero, marchand); // Appel de la fonction de vente
            }

            "2" => {
                acheter_objet(hero, marchand); // Appel de la fonction d'achat
            }

            "3" => {
                Affichage::afficher_dialogue_marchand(marchand, "end");
                save::enregistrer_hero(hero);
                save::enregistrer_marchand(marchand.id, marchand);
                break;
            }

            _ => println!("Choix invalide."),
        }
    }
}

pub fn marchandage(jeu: &mut Jeu) {
    let quartier_actuel = jeu.quartiers
        .iter_mut()
        .find(|quartier| quartier.color == jeu.quartier_actuel)
        .expect("Quartier actuel introuvable");

    let mut marchand = ini::charger_marchand_quartier(&quartier_actuel);

    discuter_avec_marchand(&mut jeu.hero, &mut marchand);
}
