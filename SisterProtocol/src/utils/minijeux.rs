use std::{collections::HashMap, fs, io::{self, Write}}; 
use anyhow::{Result, Context};
use rand::{rng, seq::IndexedMutRandom};

// Mini-jeu Mastermind Binaire
pub fn mastermind_binaire_random() -> Result<()> {
    let fichier = fs::read_to_string("assets/data/minigames/BinaryMastermind.json")
        .context("Impossible de lire binaire.json")?;
    let mut all_solutions: Vec<&str> = serde_json::from_str(&fichier)
        .context("Format JSON invalide binaire")?;

    let solution = all_solutions
        .choose_mut(&mut rng())
        .context("Pas de solution disponible")?;

    println!("[Mastermind Binaire]");
    mastermind_binaire(&solution);
    Ok(())
}

// Mini-jeu Mastermind Couleur
pub fn mastermind_couleur_random()-> Result<()> {
    // Lire le fichier JSON
    let content = fs::read_to_string("assets/data/minigames/ColoredMastermind.json")
        .context("Impossible de lire couleur.json")?;

    // Désérialiser le contenu JSON dans un vecteur de paires de couleurs
    let mut all_solutions: Vec<Vec<String>> = serde_json::from_str(&content)
        .context("Format JSON invalide couleur")?;

    // Créer un générateur RNG
    let mut rng_gen = rng();

    // Choisir une solution aléatoire parmi les solutions disponibles
    let solution = all_solutions
        .choose_mut(&mut rng_gen)
        .context("Pas de combinaison disponible")?;

    println!("[Mastermind Couleur]");

    // Passer la référence de la solution à la fonction mastermind_couleur
    mastermind_couleur(&solution);

    Ok(())
}

// Mini-jeu Pendu
pub fn pendu_random() -> Result<()> {
    let content = fs::read_to_string("assets/data/minigames/hangman.json")
        .context("Impossible de lire pendu.json")?;
    let mut mots: Vec<String> = serde_json::from_str(&content)
        .context("Format JSON invalide pendu")?;

    let mot = mots
        .choose_mut(&mut rng())
        .context("Aucun mot disponible pour le pendu")?;

    println!("[Jeu du Pendu] Le mot contient {} lettres", mot.len());
    pendu(mot);

    Ok(())
}

// --- Implémentations des jeux ---

fn mastermind_binaire(solution: &str) {
    let len = solution.len();
    let mut tentative = String::new();
    let mut essais = 10;

    println!("Devine la séquence binaire de {} bits (max {} essais)", len, essais);

    while essais > 0 {
        print!("({} restant) > ", essais);
        io::stdout().flush().unwrap();
        tentative.clear();
        io::stdin().read_line(&mut tentative).unwrap();

        let guess = tentative.trim();
        if guess.len() != len || !guess.chars().all(|c| c == '0' || c == '1') {
            println!("⛔ Entrée invalide. Entrez une séquence binaire de {} bits.", len);
            continue;
        }

        let exact = solution
            .chars()
            .zip(guess.chars())
            .filter(|(a, b)| a == b)
            .count();

        if exact == len {
            println!("\x1b[32m✅ Gagné ! La solution était bien {}\x1b[0m", solution);
            return;
        } else {
            // Coloration du retour
            let mut colored_guess = String::new();
            for (s, g) in solution.chars().zip(guess.chars()) {
                if s == g {
                    colored_guess.push_str(&format!("\x1b[32m{}\x1b[0m", g)); // Vert
                } else {
                    colored_guess.push_str(&format!("\x1b[31m{}\x1b[0m", g)); // Rouge
                }
            }
            println!("🔢 {} bien placé(s) → {}", exact, colored_guess);
        }

        essais -= 1;
    }

    println!("💀 Perdu ! La solution était : {}", solution);
}

fn mastermind_couleur(solution: &Vec<String>) {
    let len = solution.len();
    let mut tentative = String::new();
    let mut essais = 10;

    println!("Devine la combinaison de {} couleur(s) (max {} essais)", len, essais);

    while essais > 0 {
        print!("({} restant) > ", essais);
        io::stdout().flush().unwrap();  // S'assurer que le prompt est bien affiché
        tentative.clear();
        io::stdin().read_line(&mut tentative).unwrap();

        let guess: Vec<String> = tentative.trim().split_whitespace().map(|s| s.to_string()).collect();

        // Vérification que l'utilisateur a entré le bon nombre de couleurs
        if guess.len() != len {
            println!("⛔ Entrée invalide. Il faut entrer exactement {} couleur(s).", len);
            continue;
        }

        // Calcul des couleurs bien placées
        let exact = solution.iter().zip(guess.iter()).filter(|(a, b)| a == b).count();

        // Calcul des couleurs présentes mais mal placées
        let mut solution_map = HashMap::new();
        let mut guess_map = HashMap::new();

        // Comptabilisation des couleurs dans la solution et la tentative
        for i in 0..len {
            *solution_map.entry(&solution[i]).or_insert(0) += 1;
            *guess_map.entry(&guess[i]).or_insert(0) += 1;
        }


        // Si la tentative est correcte
        if exact == len {
            println!("🎉 Bravo ! La combinaison était : {:?}", solution);
            return;
        } else {
            // Affichage des couleurs avec formatage : rouge pour incorrect, vert pour correct
            print!("Essai : ");
            for (i, color) in guess.iter().enumerate() {
                if solution[i] == *color {
                    // Vert si la couleur est bien placée
                    print!("\x1b[32m{}\x1b[0m ", color);  // Vert
                } else {
                    // Rouge si la couleur est incorrecte
                    print!("\x1b[31m{}\x1b[0m ", color);  // Rouge
                }
            }
            println!();
        }

        essais -= 1;
    }

    println!("💀 Perdu. La solution était : {:?}", solution);
}

fn pendu(mot: &str) {
    let mut essais = 10;
    let mut trouve = vec!['_'; mot.len()];
    let mut lettres_proposees = vec![];

    while essais > 0 {
        println!(
            "\nMot : {} | Lettres utilisées : {:?} | Essais restants : {}",
            trouve.iter().collect::<String>(),
            lettres_proposees,
            essais
        );

        print!("Propose une lettre : ");
        io::stdout().flush().unwrap();
        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();
        let lettre = input.trim().chars().next();

        if lettre.is_none() || !lettre.unwrap().is_alphabetic() {
            println!("⛔ Entrée invalide.");
            continue;
        }

        let lettre = lettre.unwrap().to_ascii_lowercase();
        if lettres_proposees.contains(&lettre) {
            println!("⚠️ Tu as déjà proposé '{}'.", lettre);
            continue;
        }

        lettres_proposees.push(lettre);

        if mot.contains(lettre) {
            for (i, c) in mot.chars().enumerate() {
                if c == lettre {
                    trouve[i] = lettre;
                }
            }
            if !trouve.contains(&'_') {
                println!("\n🎉 Gagné ! Le mot était : {}", mot);
                return;
            }
        } else {
            essais -= 1;
            println!("❌ Raté.");
        }
    }

    println!("\n💀 Perdu. Le mot était : {}", mot);
}
