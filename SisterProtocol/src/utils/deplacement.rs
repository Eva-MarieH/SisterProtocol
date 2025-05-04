use crate::classes::jeu::Jeu;
use std::io::{self, Write};

pub fn deplacement(jeu: &mut Jeu) {

    println!("🏠 Quel quartier veux-tu aller ?");
    println!("1. Quartier bleu");
    println!("2. Quartier vert");
    println!("3. Quartier jaune");
    println!("4. Quartier violet");
    println!("5. Quartier rouge");

    print!("> Choisis le quartier : ");
    io::stdout().flush().unwrap();

    let mut choix = String::new();
    io::stdin().read_line(&mut choix).unwrap();

    let destination = match choix.trim() {
        "1" => "bleu",
        "2" => "vert",
        "3" => "jaune",
        "4" => "violet",
        "5" => "rouge",
        _ => {
            println!("⛔ Quartier inconnu.");
            return;
        }
    };

    if !peut_se_deplacer(jeu, destination) {
        return;
    }

    jeu.hero.position = destination.to_string();
    jeu.quartier_actuel = destination.to_string();
    println!("🚶 Tu marches vers le quartier {}...", destination);
}

fn peut_se_deplacer(jeu: &Jeu, destination: &str) -> bool {
    let ordre_quartiers = vec!["bleu", "vert", "jaune", "violet", "rouge"];

    let index_actuel = ordre_quartiers.iter().position(|&q| q == jeu.quartier_actuel);
    let index_destination = ordre_quartiers.iter().position(|&q| q == destination);

    match (index_actuel, index_destination) {
        (Some(i_actuel), Some(i_dest)) if i_dest == i_actuel + 1 => {
            // Rechercher le quartier actuel par couleur dans un Vec
            let quartier_actuel = jeu
                .quartiers
                .iter()
                .find(|q| q.color == jeu.quartier_actuel);

            if let Some(q) = quartier_actuel {
                if q.server.is_some() {
                    println!(
                        "⛔ Le serveur de '{}' bloque l'accès à '{}'.",
                        q.color, destination
                    );
                    return false;
                }
                true
            } else {
                println!("⚠️ Erreur : quartier actuel introuvable.");
                false
            }
        }
        (Some(i_actuel), Some(i_dest)) if i_dest == i_actuel => {
            println!("ℹ️ Tu es déjà dans le quartier '{}'.", destination);
            false
        }
        _ => {
            println!(
                "⛔ Tu ne peux pas aller à '{}' depuis '{}'. Suis l'ordre : bleu → vert → jaune → violet → rouge.",
                destination, jeu.quartier_actuel
            );
            false
        }
    }
}
