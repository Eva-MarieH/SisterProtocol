use std::fs::{File, OpenOptions};
use std::io::Read;
use serde_json;
use crate::classes::personnage::{Hero, Marchand, PNJs};
use crate::classes::quartier::Quartier;

pub fn enregistrer_hero(hero: &Hero) {
    // Ouvre ou crée le fichier Player.json pour y écrire
    let file = File::create("assets/saves/Player.json")
        .map_err(|e| format!("Erreur lors de la création du fichier Player.json : {}", e)).unwrap();
    
    // Sérialiser l'objet hero en JSON et écrire dans le fichier
    serde_json::to_writer_pretty(file, hero)
        .map_err(|_e| format!("Erreur lors de la sérialisation du hero")).unwrap();
}

pub fn enregistrer_marchand(marchand_id: u8, new_marchand: &Marchand) {
    // Charger le fichier PNJs.json
    let mut file =  File::create("assets/saves/PNJs.json")
        .map_err(|e| format!("Erreur lors de la création du fichier PNJs.json : {}", e)).unwrap();


    let mut contenu = String::new();
    file.read_to_string(&mut contenu).unwrap();

    // Désérialiser le contenu JSON dans une structure PNJs
    let mut pnjs: PNJs = match serde_json::from_str(&contenu) {
        Ok(p) => p,
        Err(_) => {
            // Si le fichier est vide ou mal formaté, on initialise une nouvelle structure PNJs
            PNJs { merchants: Vec::new(), residents: Vec::new() }
        }
    };

    // Trouver le marchand avec le bon ID et mettre à jour ses données
    if let Some(marchand) = pnjs.merchants.iter_mut().find(|m| m.id == marchand_id) {
        // Remplacer le marchand existant par le nouveau marchand
        *marchand = new_marchand.clone();
    } else {
        // Si le marchand n'existe pas, ajouter le nouveau marchand
        pnjs.merchants.push(new_marchand.clone());
    }

    // Ouvrir le fichier en mode écriture, écraser son contenu existant
    let file = OpenOptions::new()
        .write(true)
        .truncate(true)  // Efface l'ancien contenu du fichier
        .open("assets/saves/PNJs.json").unwrap();

    // Sérialiser et réécrire les données mises à jour dans le fichier PNJs.json
    serde_json::to_writer_pretty(file, &pnjs)
        .map_err(|_e| format!("Erreur lors de la sérialisation des pnj")).unwrap();
}

pub fn enregistrer_quartiers(quartiers: &Vec<Quartier>) {
    // Ouvre ou crée le fichier Quartiers.json pour y écrire
    let file = File::create("assets/saves/Districts.json")
        .map_err(|e| format!("Erreur lors de la création du fichier Districts.json : {}", e)).unwrap();
    
    // Sérialiser l'objet quartiers en JSON et écrire dans le fichier
    serde_json::to_writer_pretty(file, quartiers)
        .map_err(|e| format!("Erreur lors de la sérialisation des quartiers : {}", e)).unwrap();
}