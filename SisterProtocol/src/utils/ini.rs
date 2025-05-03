use std::fs;
use std::fs::File;
use std::io::BufReader;
use std::path::Path;
use anyhow::Result;

use crate::classes::quartier::Quartier;
use crate::classes::personnage::Hero;
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

fn charger_quartiers() -> Result<Vec<Quartier>> {
    let file = File::open("assets/saves/District.json")?;
    let reader = BufReader::new(file);
    let quartiers = serde_json::from_reader(reader)?;
    Ok(quartiers)
}

fn charger_hero() -> Result<Hero> {
    let file = File::open("assets/saves/player.json")?;
    let reader = BufReader::new(file);
    let hero = serde_json::from_reader(reader)?;
    Ok(hero)
}

pub fn initialiser_jeu() -> Result<Jeu> {
    copier_dossier_data().expect("Erreur lors de la copie du dossier data");

    let quartiers = charger_quartiers()?;
    let hero = charger_hero()?;

    let jeu = Jeu {
        quartiers,
        hero,
    };

    Ok(jeu)
}
