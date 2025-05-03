use crate::classes::inventaire::{Objet, TypeObjet};
use crate::classes::personnage::{Hero, Marchand};
use std::io::{self, Write};

pub fn marchandage(hero: &mut Hero, marchand: &mut Marchand) {
    println!("{}: {}", marchand.nom, dialogue_marchand(&marchand, "start"));
    

    loop {
        println!("\nQue veux-tu faire ?");
        println!("1. Vendre des objets");
        println!("2. Acheter des objets");
        println!("3. Partir");

        print!("> ");
        io::stdout().flush().unwrap();

        let mut choix = String::new();
        io::stdin().read_line(&mut choix).unwrap();

        match choix.trim() {
            "1" => vendre(hero, marchand),
            "2" => acheter(hero, marchand),
            "3" => {
                println!("{}: {}", marchand.nom, dialogue_marchand(&marchand, "end"));
                break;
            }
            _ => println!("Choix invalide."),
        }
    }
}

fn vendre(hero: &mut Hero, marchand: &mut Marchand) {
    let objets_vendables: Vec<_> = hero
        .inventaire
        .iter()
        .filter(|obj| matches!(obj.type_objet, TypeObjet::Nourriture | TypeObjet::Amelioration))
        .collect();

    if objets_vendables.is_empty() {
        println!("Tu n'as rien à vendre.");
        return;
    }

    println!("\nObjets à vendre :");
    for (i, obj) in objets_vendables.iter().enumerate() {
        println!("{}. {} ({} crédits)", i + 1, obj.nom, obj.prix);
    }

    print!("Quel objet veux-tu vendre ? (numéro ou '0' pour annuler) > ");
    io::stdout().flush().unwrap();
    let mut choix = String::new();
    io::stdin().read_line(&mut choix).unwrap();

    if let Ok(index) = choix.trim().parse::<usize>() {
        if index == 0 {
            return;
        }
        if index <= objets_vendables.len() {
            let obj = objets_vendables[index - 1];
            if marchand.money < obj.prix {
                println!("Le marchand est trop pauvre pour acheter cet objet.");
                return;
            }

            // Retirer l'objet du joueur
            if let Some(obj_to_remove) = hero.inventaire.drain_filter(|x| x.id == obj.id).next() {
                hero.argent += obj_to_remove.prix;
                marchand.inventory.push(obj_to_remove);
                marchand.money -= obj_to_remove.prix;
                println!("Tu as vendu {} pour {} crédits.", obj_to_remove.nom, obj_to_remove.prix);
            }
        } else {
            println!("Index invalide.");
        }
    }
}

fn acheter(hero: &mut Hero, marchand: &mut Marchand) {
    let objets_disponibles: Vec<_> = marchand
        .inventory
        .iter()
        .filter(|obj| matches!(obj.type_objet, TypeObjet::Nourriture | TypeObjet::Amelioration))
        .collect();

    if objets_disponibles.is_empty() {
        println!("Le marchand n'a rien à vendre.");
        return;
    }

    println!("\nObjets à acheter :");
    for (i, obj) in objets_disponibles.iter().enumerate() {
        println!("{}. {} ({} crédits)", i + 1, obj.nom, obj.prix);
    }

    print!("Quel objet veux-tu acheter ? (numéro ou '0' pour annuler) > ");
    io::stdout().flush().unwrap();
    let mut choix = String::new();
    io::stdin().read_line(&mut choix).unwrap();

    if let Ok(index) = choix.trim().parse::<usize>() {
        if index == 0 {
            return;
        }
        if index <= objets_disponibles.len() {
            let obj = objets_disponibles[index - 1];
            if hero.argent < obj.prix {
                println!("Tu es trop pauvre pour acheter cet objet.");
                return;
            }

            // Retirer l'objet du marchand
            if let Some(pos) = marchand.inventory.iter().position(|x| x.id == obj.id) {
                marchand.inventory.remove(pos);
                marchand.money += obj.prix;

                hero.inventaire.push(obj.clone());
                hero.argent -= obj.prix;

                println!("Tu as acheté {} pour {} crédits.", obj.nom, obj.prix);
            }
        } else {
            println!("Index invalide.");
        }
    }
}

fn dialogue_marchand(marchand: &Marchand, context: &str) -> String {
    // Remplace cette fonction par ton vrai système de dialogue si nécessaire
    match context {
        "start" => "Bienvenue, j’ai de quoi te transformer en monstre… intéressé ?".to_string(),
        "end" => "Reviens quand t’auras plus de fric… ou plus de besoins.".to_string(),
        _ => "".to_string(),
    }
}
