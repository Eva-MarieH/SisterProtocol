use std::fs::{self, File};
use std::io::BufReader;
use std::path::Path;
use serde_json::Result;
use anyhow::Context;

use crate::classes::quartier::Quartier;
use crate::classes::personnage::{Hero, Marchand, Resident};
use crate::classes::jeu::Jeu;
use crate::classes::inventaire::{Objet, Inventaire};

/// Copie le dossier `data/` vers `saves/`, en recréant le dossier à neuf
fn copier_dossier_data() -> std::io::Result<()> {
    let source = Path::new("data");
    let destination = Path::new("saves");

    if destination.exists() {
        fs::remove_dir_all(&destination)?; // On nettoie d'abord
    }
    fs::create_dir_all(&destination)?;

    for entry in fs::read_dir(source)? {
        let entry = entry?;
        let file_type = entry.file_type()?;

        if file_type.is_file() {
            let file_name = entry.file_name();
            fs::copy(entry.path(), destination.join(file_name))?;
        }
    }

    Ok(())
}

/// Charge les objets
pub fn charger_objets() -> Result<Vec<Objet>> {
    let contenu = fs::read_to_string("saves/Objects.json")
        .context("Impossible de lire le fichier Objects.json").unwrap();

    let objets: Vec<Objet> = serde_json::from_str(&contenu)
        .context("Erreur lors du parsing de Objects.json").unwrap();

    Ok(objets)
}

/// Charge les quartiers (chaque Quartier contient déjà ses PNJ, ennemis, etc.)
fn charger_quartiers() -> Result<Vec<Quartier>> {
    let file = File::open("saves/District.json").unwrap();
    let reader = BufReader::new(file);
    serde_json::from_reader(reader)
}

// Charger le quartier en fonction de la position du héros
pub fn charger_quartier(hero: &Hero) -> Result<Option<Quartier>> {
    // Lire le fichier districts.json
    let contenu = fs::read_to_string("assets/districts.json")
        .context("Erreur de lecture de districts.json").unwrap();

    // Désérialiser en un Vec<QuartierBrut>
    let quartiers: Vec<Quartier> = serde_json::from_str(&contenu)
        .context("Erreur de parsing de districts.json").unwrap();

    // Rechercher le quartier correspondant à la couleur du héros
    let quartier = quartiers
        .into_iter()
        .find(|q| q.color == hero.position);

    Ok(quartier)
}

// Charger les résidents du quartier
pub fn charger_residents_quartier(quartier: &Quartier) -> Result<Vec<Resident>> {
    // Lire les résidents depuis le fichier JSON
    let contenu_residents = fs::read_to_string("assets/PNJs.json")
        .context("Erreur de lecture de PNJs.json").unwrap();
    let residents: Vec<Resident> = serde_json::from_str(&contenu_residents)
        .context("Erreur de parsing de PNJs.json").unwrap();

    // Filtrer les résidents en fonction des IDs du quartier
    let citoyens = residents.into_iter()
        .filter(|r| quartier.residents.contains(&(r.id as usize)))
        .collect();

    Ok(citoyens)
}

// Charger le marchand du quartier
pub fn charger_marchand_quartier(quartier: &Quartier) -> Result<Option<Marchand>> {
    // Lire les PNJs depuis le fichier JSON
    let contenu = fs::read_to_string("assets/PNJs.json")
        .context("Erreur de lecture de PNJs.json").unwrap();

    let marchands: Vec<Marchand> = serde_json::from_str(&contenu)
        .context("Erreur de parsing de PNJs.json").unwrap();

    // Rechercher le marchand avec l'id correspondant
    let marchand = marchands
        .into_iter()
        .find(|m| m.id == quartier.merchant);

    Ok(marchand)
}

/// Charge le héros
fn charger_hero() -> Result<Hero> {
    let file = File::open("saves/player.json").unwrap();
    let reader = BufReader::new(file);
    serde_json::from_reader(reader)
}

/// Fonction principale d'initialisation du jeu
pub fn initialiser_jeu() -> Result<Jeu> {
    copier_dossier_data().expect("Erreur lors de la copie du dossier data");

    let quartiers = charger_quartiers()?;
    let hero = charger_hero()?;

    // Tu peux ici extraire le quartier actuel du héros si tu le stockes dans Hero :
    let quartier_actuel = hero.position.clone(); // ou un autre champ, selon ta struct

    let jeu = Jeu {
        quartiers,
        quartier_actuel,
        hero,
    };

    Ok(jeu)
}