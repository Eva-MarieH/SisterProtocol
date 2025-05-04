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
    let quartiers = serde_json::from_reader(reader)?;
    Ok(quartiers)
}


// Charger le quartier en fonction de la position du héros
pub fn charger_quartier(hero: &Hero) -> Result<Quartier> {
    let file = File::open("saves/District.json").context("Échec ouverture de District.json")?;
    let reader = BufReader::new(file);
    let quartiers: Vec<Quartier> = serde_json::from_reader(reader).context("Échec de parsing de District.json")?;

    quartiers
        .into_iter()
        .find(|q| q.color == hero.position)
        .ok_or_else(|| anyhow::anyhow!("Aucun quartier trouvé pour la position: {}", hero.position))
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
        .filter(|r| quartier.residents.contains(&(r.id as u8)))
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
    let hero = serde_json::from_reader(reader)?;
    Ok(hero)
}

pub fn initialiser_jeu() -> Jeu {
    copier_dossier_data().expect("Erreur lors de la copie du dossier data");

    let quartiers = charger_quartiers().unwrap();
    let hero = charger_hero().unwrap();
    let quartier_actuel = hero.position.clone();

    let jeu = Jeu {
        quartiers,
        quartier_actuel,
        hero

    };

    jeu
}
