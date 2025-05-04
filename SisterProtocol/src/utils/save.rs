use std::fs::{File, OpenOptions};
use std::io::Read;
use serde_json;
use crate::classes::personnage::{Boss, Hero, Marchand, PNJs};
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
    // Lire le fichier existant
    let mut contenu = String::new();

    if let Ok(mut file_lecture) = File::open("assets/saves/PNJs.json") {
        file_lecture.read_to_string(&mut contenu).unwrap();
    }

    // Désérialiser ou initialiser si erreur
    let mut pnjs: PNJs = serde_json::from_str(&contenu).unwrap_or_else(|_| PNJs {
        merchants: Vec::new(),
        residents: Vec::new(),
        guards: Vec::new(),
        boss: Boss {
            name: "N/A".to_string(),
            alias: "N/A".to_string(),
            force: 0,
        },
    });

    // Ajouter ou mettre à jour le marchand
    if let Some(marchand) = pnjs.merchants.iter_mut().find(|m| m.id == marchand_id) {
        *marchand = new_marchand.clone();
    } else {
        pnjs.merchants.push(new_marchand.clone());
    }

    // Écriture dans le fichier (troncature)
    let file_ecriture = OpenOptions::new()
        .write(true)
        .truncate(true)
        .create(true)
        .open("assets/saves/PNJs.json")
        .expect("Erreur lors de l'ouverture du fichier en écriture");

    serde_json::to_writer_pretty(file_ecriture, &pnjs)
        .expect("Erreur lors de la sérialisation des PNJs");
}

pub fn enregistrer_quartiers(quartiers: &Vec<Quartier>) {
    // Ouvre ou crée le fichier Quartiers.json pour y écrire
    let file = File::create("assets/saves/District.json")
        .map_err(|e| format!("Erreur lors de la création du fichier District.json : {}", e)).unwrap();
    
    // Sérialiser l'objet quartiers en JSON et écrire dans le fichier
    serde_json::to_writer_pretty(file, quartiers)
        .map_err(|e| format!("Erreur lors de la sérialisation des quartiers : {}", e)).unwrap();
}