use std::fs;
use std::io::{self, Write};
use anyhow::{Result, Context};

use crate::classes::inventaire::{Objet, TypeObjet, ObjetQuantifie, Inventaire};
use crate::classes::personnage::{Hero, Marchand};
use crate::classes::quartier::{self, Quartier};
use crate::utils::affichage::Affichage;
use crate::utils::ini;


fn ajouter_a_inventaire(inventaire: &mut Inventaire, id: u8, quantite: u32) {
    if let Some(obj) = inventaire.objets.iter_mut().find(|o| o.id == id) {
        obj.quantity += quantite;
    } else {
        inventaire.objets.push(ObjetQuantifie { id, quantity: quantite });
    }
}


fn retirer_du_inventaire(inventaire: &mut Inventaire, id: u8, quantite: u32) -> bool {
    if let Some(pos) = inventaire.objets.iter().position(|o| o.id == id && o.quantity >= quantite) {
        let obj = &mut inventaire.objets[pos];
        obj.quantity -= quantite;
        if obj.quantity == 0 {
            inventaire.objets.remove(pos);
        }
        true
    } else {
        false
    }
}

pub fn discuter_avec_marchand(hero: &mut Hero, marchand: &mut Marchand) {
    let objets = match ini::charger_objets() {
        Ok(objs) => objs,
        Err(e) => {
            println!("Erreur lors du chargement des objets : {}", e);
            return;
        }
    };
    
    Affichage::afficher_dialogue_marchand(marchand, "start");

    loop {
        println!("\nQue veux-tu faire ?");
        println!("1. Vendre");
        println!("2. Acheter");
        println!("3. Partir");

        print!("> ");
        io::stdout().flush().unwrap();
        let mut choix = String::new();
        io::stdin().read_line(&mut choix).unwrap();

        match choix.trim() {
            "1" => {
                println!("\n--- Tes objets à vendre ---");
                Affichage::afficher_inventaire_filtre(&hero.inventaire.objets, &objets, true);
                print!("ID de l’objet à vendre : ");
                io::stdout().flush().unwrap();
                let mut id_str = String::new();
                io::stdin().read_line(&mut id_str).unwrap();
                let id: u8 = match id_str.trim().parse() {
                    Ok(val) => val,
                    Err(_) => { println!("Entrée invalide."); continue; }
                };

                if let Some(obj) = Affichage::trouver_objet_par_id(id, &objets) {
                    if !matches!(obj.type_objet, TypeObjet::Nourriture | TypeObjet::Amelioration) {
                        println!("Tu ne peux pas vendre cet objet.");
                        continue;
                    }

                    if retirer_du_inventaire(&mut hero.inventaire, id, 1) {
                        if marchand.money >= obj.prix {
                            hero.argent += obj.prix;
                            marchand.money -= obj.prix;
                            ajouter_a_inventaire(&mut marchand.inventory, id, 1);
                            println!("Vendu pour {} crédits.", obj.prix);
                        } else {
                            println!("Vendeur trop pauvre.");
                            ajouter_a_inventaire(&mut hero.inventaire, id, 1); // Rembourse
                        }
                    } else {
                        println!("Tu ne possèdes pas cet objet.");
                    }
                } else {
                    println!("Objet introuvable.");
                }
            }

            "2" => {
                println!("\n--- Objets du marchand ---");
                Affichage::afficher_inventaire(&marchand.inventory);
                print!("ID de l’objet à acheter : ");
                io::stdout().flush().unwrap();
                let mut id_str = String::new();
                io::stdin().read_line(&mut id_str).unwrap();
                let id: u8 = match id_str.trim().parse() {
                    Ok(val) => val,
                    Err(_) => { println!("Entrée invalide."); continue; }
                };

                if let Some(obj) = Affichage::trouver_objet_par_id(id, &objets) {
                    if !matches!(obj.type_objet, TypeObjet::Nourriture | TypeObjet::Amelioration) {
                        println!("Tu ne peux pas acheter cet objet.");
                        continue;
                    }

                    if hero.argent < obj.prix {
                        println!("Vous êtes trop pauvre.");
                        continue;
                    }

                    if retirer_du_inventaire(&mut marchand.inventory, id, 1) {
                        hero.argent -= obj.prix;
                        marchand.money += obj.prix;
                        ajouter_a_inventaire(&mut hero.inventaire, id, 1);
                        println!("Achat réussi !");
                    } else {
                        println!("Objet non disponible chez le marchand.");
                    }
                } else {
                    println!("Objet introuvable.");
                }
            }

            "3" => {
                Affichage::afficher_dialogue_marchand(marchand, "end");
                break;
            }

            _ => println!("Choix invalide."),
        }
    }
}


pub fn marchandage(hero: &mut Hero) {
    match ini::charger_quartier(hero) {
        Ok(Some(q)) => {
            match ini::charger_marchand_quartier(&q) {
                Ok(Some(mut marchand)) => {
                    discuter_avec_marchand(hero, &mut marchand);
                }
                Ok(None) => println!("Il n'y a pas de marchand dans ce quartier."),
                Err(e) => println!("Erreur lors du chargement du marchand : {}", e),
            }
        }
        Ok(None) => println!("Quartier introuvable."),
        Err(e) => println!("Erreur lors du chargement du quartier : {}", e),
    }
}


