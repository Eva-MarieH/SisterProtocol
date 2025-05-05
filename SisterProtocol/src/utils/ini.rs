use std::fs::{self, File};
use std::io::BufReader;
use std::path::Path;
use anyhow::{Context, Result};

use crate::classes::entites::{Machines, Ordinateur, Serveur};
use crate::classes::quartier::Quartier;
use crate::classes::personnage::{Garde, Hero, Marchand, PNJs, Resident, Boss};
use crate::classes::inventaire::Objet;
use crate::classes::lore::Lore;
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
pub fn charger_residents_quartier(quartier: &Quartier) -> Vec<Resident> {
    let contenu_residents = fs::read_to_string("assets/saves/PNJs.json")
        .context("Erreur de lecture de PNJs.json").unwrap();

    let pnj_data: PNJs = serde_json::from_str(&contenu_residents)
        .context("Erreur de parsing de PNJs.json").unwrap();

    let citoyens = pnj_data.residents.into_iter()
        .filter(|r| quartier.residents.contains(&(r.id as u8)))
        .collect();

    citoyens
}


// Charger le marchand du quartier
pub fn charger_marchand_quartier(quartier: &Quartier) -> Marchand {
    let contenu = fs::read_to_string("assets/saves/PNJs.json")
        .context("Erreur de lecture de PNJs.json").unwrap();

    let pnj_data: PNJs = serde_json::from_str(&contenu)
        .context("Erreur de parsing de PNJs.json").unwrap();

    pnj_data
        .merchants
        .into_iter()
        .find(|m| m.id == quartier.merchant)
        .expect("Aucun marchand correspondant trouvé pour ce quartier")
}

// Charger le premier garde du quartier
pub fn charger_premier_garde_quartier(quartier: &Quartier) -> Option<Garde> {
    let contenu_garde = fs::read_to_string("assets/saves/PNJs.json")
        .context("Erreur de lecture de PNJs.json").unwrap();

    let pnj_data: PNJs = serde_json::from_str(&contenu_garde)
        .context("Erreur de parsing de PNJs.json").unwrap();

    let ids = match quartier.guards.as_ref(){
        Some(ids) => ids,
        None => return None
    };

    // Filtrer les gardes en fonction des ids
    let guards: Vec<Garde> = pnj_data
        .guards
        .into_iter()
        .filter(|guard| ids.contains(&guard.id))
        .collect();

    // Retourner le premier garde, ou None si aucun garde n'est trouvé
    guards.first().cloned()
}

// Charger le boss du quartier
pub fn charger_boss_quartier() -> Boss{
    let contenu = fs::read_to_string("assets/saves/PNJs.json")
        .context("Erreur de lecture de PNJs.json").unwrap();

    let pnj_data: PNJs = serde_json::from_str(&contenu)
        .context("Erreur de parsing de PNJs.json").unwrap();

    pnj_data.boss
}


// Charger le serveur du quartier
pub fn charger_serveur_quartier(quartier: &Quartier) -> Option<Serveur> {
    let contenu = fs::read_to_string("assets/saves/Ennemies.json")
        .context("Erreur de lecture de Ennemies.json").unwrap();

    let machines: Machines = serde_json::from_str(&contenu)
        .context("Erreur de parsing de Ennemies.json").unwrap();

    let server_id = quartier.server.expect("Aucun serveur");

    machines
        .servers
        .into_iter()
        .find(|m| m.id == server_id)
}

// Charger le premier ordinateur du quartier
pub fn charger_premier_ordinateur_quartier(quartier: &Quartier) -> Option<Ordinateur> {
    let contenu_ordinateurs = fs::read_to_string("assets/saves/Ennemies.json")
        .context("Erreur de lecture de Ennemies.json").unwrap();

    let machines: Machines = serde_json::from_str(&contenu_ordinateurs)
        .context("Erreur de parsing de Ennemies.json").unwrap();

    let ids = match quartier.ordinateurs.as_ref(){
        Some(ids) => ids,
        None => return None
    };

    // Filtrer les ordinateurs en fonction des ids
    let ordinateurs: Vec<Ordinateur> = machines
        .computers
        .into_iter()
        .filter(|ordi| ids.contains(&ordi.id))
        .collect();

    // Retourner le premier ordinateur, ou None si aucun ordinateur n'est trouvé
    ordinateurs.first().cloned()
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
    let lore = charger_lore();

    Jeu {
        quartiers,
        quartier_actuel,
        hero,
        lore
    }
    
    
}
pub fn charger_lore() -> Lore {
    let contenu = fs::read_to_string("assets/saves/Lore.json")
        .expect("Impossible de lire le fichier de Lore.json.");
    serde_json::from_str(&contenu)
        .expect("Erreur lors du parsing du fichier de Lore.json.")
}

pub fn initialiser_jeu() -> Jeu {
    copier_dossier_data().expect("Erreur lors de la copie du dossier data");
    charger_donnees()
}

pub fn continue_jeu() -> Jeu {
    charger_donnees()
}
