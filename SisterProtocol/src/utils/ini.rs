use std::fs::{self, File};
use std::io::BufReader;
use std::path::Path;
use anyhow::{Context, Result};

use crate::classes::quartier::Quartier;
use crate::classes::personnage::{Hero, Resident, Marchand};
use crate::classes::inventaire::Objet;
use crate::classes::jeu::Jeu;

fn copier_dossier_data() -> std::io::Result<()> {
    let source = Path::new("assets/data");
    let destination = Path::new("assets/saves");

    if destination.exists() {
        fs::remove_dir_all(&destination)?;
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
    let contenu = fs::read_to_string("assets/saves/Objects.json")
        .context("Impossible de lire le fichier Objects.json")?;

    let objets: Vec<Objet> = serde_json::from_str(&contenu)
        .context("Erreur lors du parsing de Objects.json")?;

    Ok(objets)
}

/// Charge les quartiers (chaque Quartier contient déjà ses PNJ, ennemis, etc.)
fn charger_quartiers() -> Result<Vec<Quartier>> {
    let file = File::open("assets/saves/District.json")
        .context("Erreur lors de l'ouverture du fichier District.json")?;
    let reader = BufReader::new(file);
    let quartiers = serde_json::from_reader(reader)
        .context("Erreur lors du parsing de District.json")?;
    Ok(quartiers)
}

// Charger les résidents du quartier
pub fn charger_residents_quartier(quartier: &Quartier) -> Result<Vec<Resident>> {
    let contenu_residents = fs::read_to_string("assets/PNJs.json")
        .context("Erreur de lecture de PNJs.json")?;
    let residents: Vec<Resident> = serde_json::from_str(&contenu_residents)
        .context("Erreur de parsing de PNJs.json")?;

    let citoyens = residents.into_iter()
        .filter(|r| quartier.residents.contains(&(r.id as u8)))
        .collect();

    Ok(citoyens)
}

// Charger le marchand du quartier
pub fn charger_marchand_quartier(quartier: &Quartier) -> Result<Option<Marchand>> {
    let contenu = fs::read_to_string("assets/PNJs.json")
        .context("Erreur de lecture de PNJs.json")?;

    let marchands: Vec<Marchand> = serde_json::from_str(&contenu)
        .context("Erreur de parsing de PNJs.json")?;

    let marchand = marchands
        .into_iter()
        .find(|m| m.id == quartier.merchant);

    Ok(marchand)
}

/// Charge le héros
fn charger_hero() -> Result<Hero> {
    let file = File::open("assets/saves/player.json")
        .context("Erreur lors de l'ouverture du fichier player.json")?;
    let reader = BufReader::new(file);
    let hero = serde_json::from_reader(reader)
        .context("Erreur lors du parsing de player.json")?;
    Ok(hero)
}

/// Fonction pour charger les données du jeu
fn charger_donnees() -> Jeu {
    let quartiers = charger_quartiers().unwrap();
    let hero = charger_hero().unwrap();
    let quartier_actuel = hero.position.clone();

    Jeu {
        quartiers,
        quartier_actuel,
        hero
    }
}

pub fn initialiser_jeu() -> Jeu {
    copier_dossier_data().expect("Erreur lors de la copie du dossier data");
    charger_donnees()
}

pub fn continue_jeu() -> Jeu {
    charger_donnees()
}
