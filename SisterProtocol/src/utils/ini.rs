use std::fs;
use std::fs::File;
use std::io::BufReader;
use std::path::Path;
use serde_json::Result;

use crate::classes::quartier::Quartier;
use crate::classes::personnage::Hero;
use crate::Jeu;

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

/// Charge les quartiers (chaque Quartier contient déjà ses PNJ, ennemis, etc.)
fn charger_quartiers() -> Result<Vec<Quartier>> {
    let file = File::open("saves/District.json")?;
    let reader = BufReader::new(file);
    serde_json::from_reader(reader)
}

/// Charge le héros
fn charger_hero() -> Result<Hero> {
    let file = File::open("saves/player.json")?;
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
