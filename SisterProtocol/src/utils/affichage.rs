use crate::classes::jeu::Jeu;
use crate::classes::quartier::Quartier;
use crate::classes::personnage::{Resident, Marchand, Hero};
use crate::classes::inventaire::{Objet, Inventaire, TypeObjet, ObjetQuantifie};
use crate::utils::ini;

pub struct Affichage;

impl Affichage {
    pub fn afficher_message(message: &str) {
        println!("{}", message);
        // ajouter couleur en fonction de la couleur du quartier
    }
    
    pub fn afficher_jeu(jeu: &Jeu) {
        println!("\n=== État du Jeu ===");
        for quartier in &jeu.quartiers {
            println!("- Quartier: {}", quartier.color);
        }
        println!("- Héro: {}", jeu.hero.nom);
    }
    
    pub fn afficher_quartier(quartier: &Quartier) {
        println!("\n=== Quartier: {} ===", quartier.color);
        println!("Gardes: {}", quartier.enemies.len());
        match &quartier.ordinateurs {
            Some(ordinateurs) => println!("Ordinateurs: {}", ordinateurs.len()),
            None => println!("Ordinateurs: 0"),
        }
        if quartier.server.is_some() {
            println!("Serveur: Présent");
        }
        if quartier.boss.is_some() {
            println!("Boss: Présent");
        }
        println!("Marchands: Présent");
        println!("PNJs: {}", quartier.residents.len());
    }

    // Afficher les dialogues d'un résident en fonction du contexte
    pub fn afficher_dialogue_residents(resident: &Resident, context: &str) {
        // Chercher un dialogue spécifique au contexte
        if let Some(dialogue) = resident.dialogues.iter().find(|d| d.context == context) {
            println!("{}", dialogue.text); // Afficher le texte du dialogue
        } else {
            println!("Aucun dialogue pour ce contexte.");
        }
    }

    pub fn afficher_dialogue_marchand(marchand: &Marchand, context: &str) {
        // Chercher un dialogue spécifique au contexte
        if let Some(dialogue) = marchand.dialogues.iter().find(|d| d.context == context) {
            println!("{}", dialogue.text); // Afficher le texte du dialogue
        } else {
            println!("Aucun dialogue pour ce contexte.");
        }
    }

    pub fn afficher_resultat_hacking(result: &Result<(), anyhow::Error>, cible: &str) {
        match result {
            Ok(_) => println!("✅ Tu as neutralisé {}", cible),
            Err(e) => println!("💀 Échec contre {} : {}", cible, e),
        }
    }
    
    pub fn afficher_hero(hero: &Hero) {
        println!("\n=== Héro ===");
        println!("Nom: {}", hero.nom);
        println!("Vie: {}", hero.vie);
        println!("Force: {}", hero.force);
        println!("Intelligence: {}", hero.intelligence);
        println!("Argent: {}", hero.argent);
        println!("Position: {}", hero.position);
        println!("Inventaire: {} objets", hero.inventaire.objets.len());
    }



    // Fonction pour afficher les objets de l'inventaire
    pub fn afficher_inventaire(inventaire: &Inventaire) {
        let objets = match ini::charger_objets() {
            Ok(objs) => objs,
            Err(_) => {
                println!("Erreur de chargement des objets.");
                return;
            }
        };
    
        println!("[Inventaire]");
        for oq in &inventaire.objets {
            if let Some(objet) = objets.iter().find(|o| o.id == oq.id) {
                let type_str = match objet.type_objet {
                    TypeObjet::Nourriture => "Nourriture",
                    TypeObjet::Amelioration => "Amélioration",
                    TypeObjet::CarteAcces => "Carte d'accès",
                };
                println!(
                    "Nom: {}, Effet: {}, Type: {}, Quantité: {}",
                    objet.nom, objet.effet, type_str, oq.quantity
                );
            } else {
                println!("Objet inconnu avec ID {} (quantité: {})", oq.id, oq.quantity);
            }
        }
    }

    pub fn trouver_objet_par_id(id: u8, objets: &[Objet]) -> Option<&Objet> {
        objets.iter().find(|obj| obj.id == id)
    }
    
    pub fn afficher_inventaire_filtre(inventaire: &[ObjetQuantifie], objets: &[Objet], filtrer: bool) {
        for obj_q in inventaire {
            if let Some(obj) = Self::trouver_objet_par_id(obj_q.id, objets) {
                if filtrer && !matches!(obj.type_objet, TypeObjet::Nourriture | TypeObjet::Amelioration) {
                    continue;
                }
                println!("{}  x{} - {} crédits", obj.nom, obj_q.quantity, obj.prix);
            }
        }
    }

    pub fn afficher_actions() {
        println!("\n🕹️  Actions possibles :");
        println!("1. Se déplacer vers un autre quartier");
        println!("2. Utiliser un objet de l'inventaire");
        println!("3. Parler à un citoyen");
        println!("4. Marchander");
        println!("5. Hacker");
        println!("6. Combattre");
        println!("7. Quitter le jeu");
    }
    
    
}
