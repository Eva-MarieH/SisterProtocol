use rand::seq::SliceRandom;
use rand::thread_rng;
use serde::Deserialize;
use std::fs;
use anyhow::{Result, Context};
use std::collections::HashSet;
use std::io::{self, Write};

//
// Mini-jeu Mastermind Binaire
//

#[derive(Debug, Deserialize)]
struct BinaireDifficultySet {
    difficulty: u8,
    solutions: Vec<String>,
}

pub fn mastermind_binaire_difficulte() -> Result<()> {
    let fichier = fs::read_to_string("assets/minigames/binaire.json")
        .context("Impossible de lire binaire.json")?;
    let all_sets: Vec<BinaireDifficultySet> = serde_json::from_str(&fichier)
        .context("Format JSON invalide")?;

    let difficulte = 0; // imposé
    let entry = all_sets.iter()
        .find(|set| set.difficulty == difficulte)
        .context("Aucune solution pour difficulté 0")?;

    let solution = entry
        .solutions
        .choose(&mut thread_rng())
        .context("Pas de solution disponible")?;

    println!("[Mastermind Binaire - difficulté 0]");
    mastermind_binaire(solution);

    Ok(())
}

//
// Mini-jeu Mastermind Couleur
//

#[derive(Debug, Deserialize)]
struct CouleurSet {
    solutions: Vec<Vec<String>>,
}

pub fn mastermind_couleur_random() -> Result<()> {
    let content = fs::read_to_string("assets/minigames/couleur.json")
        .context("Impossible de lire couleur.json")?;
    let parsed: Vec<CouleurSet> = serde_json::from_str(&content)
        .context("Format JSON invalide")?;

    let all_solutions = &parsed[0].solutions;

    let solution = all_solutions
        .choose(&mut thread_rng())
        .context("Pas de combinaison disponible")?;

    // Déduire les couleurs possibles automatiquement
    let mut set = HashSet::new();
    for combo in all_solutions {
        for couleur in combo {
            set.insert(couleur.clone());
        }
    }
    let possibles: Vec<&str> = set.iter().map(|s| s.as_str()).collect();
    let solution_refs: Vec<&str> = solution.iter().map(|s| s.as_str()).collect();

    println!("[Mastermind Couleur]");
    mastermind_couleur(&solution_refs, &possibles);

    Ok(())
}

//
// Mini-jeu Pendu
//

pub fn pendu_random() -> Result<()> {
    let content = fs::read_to_string("assets/minigames/pendu.json")
        .context("Impossible de lire pendu.json")?;
    let mots: Vec<String> = serde_json::from_str(&content)
        .context("Format JSON invalide")?;

    let mot = mots
        .choose(&mut thread_rng())
        .context("Aucun mot disponible pour le pendu")?;

    let essais = 6;
    println!("[Jeu du Pendu] Le mot contient {} lettres", mot.len());
    pendu(mot);

    Ok(())
}

//
// Lancement générique d'un mini-jeu
//

pub fn lancer_mini_jeu(nom: &str) -> Result<()> {
    match nom {
        "binaire" => mastermind_binaire_difficulte()?,
        "couleur" => mastermind_couleur_random()?,
        "pendu" => pendu_random()?,
        _ => println!("Mini-jeu inconnu : {}", nom),
    }
    Ok(())
}

// Les fonctions ci-dessous doivent être définies quelque part :

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
            println!("✅ Gagné ! La solution était bien {}", solution);
            return;
        } else {
            println!("🔢 {} bien placé(s).", exact);
        }

        essais -= 1;
    }

    println!("💀 Perdu ! La solution était : {}", solution);
}

fn mastermind_couleur(solution: &[&str], possibles: &[&str]) {
    let len = solution.len();
    let mut tentative = String::new();
    let mut essais = 10;

    println!("Devine la combinaison de {} couleur(s) (max {} essais)", len, essais);
    println!("Couleurs possibles : {}", possibles.join(", "));

    while essais > 0 {
        print!("({} restant) > ", essais);
        io::stdout().flush().unwrap();
        tentative.clear();
        io::stdin().read_line(&mut tentative).unwrap();

        let guess: Vec<&str> = tentative.trim().split_whitespace().collect();

        if guess.len() != len || !guess.iter().all(|c| possibles.contains(c)) {
            println!("⛔ Entrée invalide. Utilise exactement {} couleurs parmi : {}", len, possibles.join(", "));
            continue;
        }

        let exact = solution.iter().zip(&guess).filter(|(a, b)| a == b).count();
        if exact == len {
            println!("🎉 Bravo ! La combinaison était : {:?}", solution);
            return;
        } else {
            println!("🎨 {} couleur(s) bien placée(s).", exact);
        }

        essais -= 1;
    }

    println!("💀 Perdu. La solution était : {:?}", solution);
}


fn pendu(mot: &str,) {
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
